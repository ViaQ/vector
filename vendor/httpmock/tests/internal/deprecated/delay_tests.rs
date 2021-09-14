extern crate httpmock;

use std::time::{Duration, SystemTime};

use isahc::get;

use httpmock::{Mock, MockServer};

#[test]
fn delay_test() {
    // Arrange
    let _ = env_logger::try_init();
    let start_time = SystemTime::now();
    let delay = Duration::from_secs(3);

    let server = MockServer::start();

    let mock = Mock::new()
        .expect_path("/delay")
        .return_with_delay(delay)
        .create_on(&server);

    // Act: Send the HTTP request
    let response = get(server.url("/delay")).unwrap();

    // Assert
    mock.assert();
    assert_eq!(response.status(), 200);
    assert_eq!(start_time.elapsed().unwrap() > delay, true);
}
