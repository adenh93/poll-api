use chrono::Utc;
use fake::faker::{
    chrono::en::{DateTimeAfter, DateTimeBefore},
    internet::en::IPv4,
    lorem::en::Sentence,
};
use fake::Fake;
use once_cell::sync::Lazy;
use poll_api::{
    config::{get_config, DatabaseSettings},
    domain::{CreatedPoll, NewPoll, NewPollChoice, Poll, PollChoice},
    errors::HttpResult,
    repositories::poll::create_new_poll_and_choices,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::Response;
use sqlx::types::ipnetwork::IpNetwork;
use sqlx::{Connection, Executor, PgConnection, PgPool, QueryBuilder};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info";
    let subscriber_name = "test";

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub client: reqwest::Client,
    pub connection_pool: PgPool,
}

impl TestApp {
    pub async fn new() -> Self {
        Lazy::force(&TRACING);

        let mut config = get_config().expect("Failed to load config");
        config.application.port = 0;
        config.database.database_name = Uuid::new_v4().to_string();

        let connection_pool = configure_database(&config.database).await;

        let application = Application::build(config, connection_pool.clone())
            .await
            .expect("Failed to build application");

        let application_port = application.port();
        let address = format!("http://127.0.0.1:{}", application_port);
        let _ = tokio::spawn(application.run_until_stopped());
        let client = reqwest::Client::new();

        Self {
            address,
            port: application_port,
            client,
            connection_pool,
        }
    }

    pub async fn get_poll(&self, uuid: &Uuid) -> Response {
        self.client
            .get(&format!("{}/polls/{}", &self.address, &uuid.to_string()))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_poll(&self, new_poll: &NewPoll) -> Response {
        self.client
            .post(&format!("{}/polls", &self.address))
            .json(new_poll)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn vote_poll(&self, poll_id: &Uuid, choice: &Uuid) -> Response {
        self.client
            .post(&format!(
                "{}/polls/{}/vote/{}",
                &self.address,
                &poll_id.to_string(),
                &choice.to_string()
            ))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_poll_results(&self, poll_id: &Uuid) -> Response {
        self.client
            .get(&format!(
                "{}/polls/{}/results",
                &self.address,
                &poll_id.to_string()
            ))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn add_past_election(&self, new_poll: &NewPoll) -> HttpResult<CreatedPoll> {
        let start_date = DateTimeBefore(new_poll.end_date).fake();

        let created_poll =
            create_new_poll_and_choices(&new_poll, &start_date, &self.connection_pool).await?;

        Ok(created_poll)
    }

    pub async fn simulate_poll(&self, poll: &Poll, number_of_votes: usize) -> sqlx::Result<()> {
        let mut rng = thread_rng();

        let mut builder = QueryBuilder::new(
            "INSERT INTO poll_vote (id, poll_id, choice_id, ip_address, created_at)",
        );

        builder.push_values(0..number_of_votes, |mut b, _| {
            let random_choice = &poll.choices.choose(&mut rng).unwrap().id;
            let ip_address: IpNetwork = IPv4().fake::<String>().parse().unwrap();

            b.push_bind(Uuid::new_v4())
                .push_bind(poll.id)
                .push_bind(random_choice)
                .push_bind(ip_address)
                .push_bind(Utc::now());
        });

        builder
            .build()
            .execute(&self.connection_pool)
            .await
            .map_err(|err| {
                tracing::error!("Failed to execute query: {err:?}");
                err
            })?;

        Ok(())
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create test database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

pub fn generate_poll(number_of_choices: usize, in_past: bool) -> NewPoll {
    let fake_sentence = Sentence(5..8);

    let choices = (0..number_of_choices)
        .map(|_| NewPollChoice {
            name: fake_sentence.fake(),
        })
        .collect();

    let end_date = match in_past {
        true => DateTimeBefore(Utc::now()).fake(),
        false => DateTimeAfter(Utc::now()).fake(),
    };

    NewPoll {
        name: fake_sentence.fake(),
        description: fake_sentence.fake(),
        end_date,
        choices,
    }
}

pub fn pick_random_choice(choices: &Vec<PollChoice>) -> Uuid {
    let mut rng = thread_rng();
    choices.choose(&mut rng).unwrap().id
}
