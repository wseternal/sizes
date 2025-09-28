use sizes::StaticBox;
use std::time::Duration;
use testcontainers::{
    clients::Cli,
    core::{Port, WaitFor},
    Container, GenericImage, RunnableImage,
};
use tokio::{runtime::Runtime, sync::OnceCell};

const SURREAL_DB_PORT: u16 = 8000;

pub struct TestFixture {
    pub container: Container<'static, GenericImage>,
    docker: &'static Cli,
}

impl TestFixture {
    pub fn stop(&self) {
        self.container.stop();
        self.container.rm();
        StaticBox::drop_raw(self.docker);
    }
}

pub async fn get_test_fixture() -> &'static TestFixture {
    static _TEST_FIXTURE: OnceCell<TestFixture> = OnceCell::const_new();

    _TEST_FIXTURE
        .get_or_init(|| async {
            let cli = StaticBox::new(Cli::default());
            let docker = cli.get();
            let base = GenericImage::new("surrealdb/surrealdb", "v2.0.2").with_wait_for(
                WaitFor::Duration {
                    length: Duration::from_secs(4),
                },
            );
            let image = RunnableImage::from((base, vec!["start".to_string(), "--unauthenticated".to_string()]))
                .with_mapped_port(Port::from((0, SURREAL_DB_PORT)));
            let db_container = docker.run(image);
            db_container.start();
            let port = db_container
                .ports()
                .map_to_host_port_ipv4(SURREAL_DB_PORT)
                .unwrap();
            println!("mapped port is {:?}", port);
            TestFixture {
                container: db_container,
                docker,
            }
        })
        .await
}


pub fn get_tokio_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap()
}
