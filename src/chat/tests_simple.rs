#[cfg(test)]
mod tests {
    use mockito;

    #[test]
    fn test_mockito_basic() {
        // Create a mock
        let m = mockito::mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("hello world")
            .create();

        // Perform a request to the mock
        let url = format!("{}/", mockito::server_url());
        let response = reqwest::blocking::get(&url).unwrap();

        // Assert the response
        assert_eq!(response.status(), 200);
        assert_eq!(response.text().unwrap(), "hello world");

        // Verify the mock was called
        m.assert();
    }
}