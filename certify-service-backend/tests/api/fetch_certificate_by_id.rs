use crate::helpers::TestApp;

const VALID_CERT_PEM: &str = r#"-----BEGIN CERTIFICATE-----
MIIDuzCCAqOgAwIBAgIUEUYshp4fKoXtb+SUxcC7nm2dSAcwDQYJKoZIhvcNAQEL
BQAwPjEZMBcGA1UEAwwQdGVzdC5leGFtcGxlLmNvbTEUMBIGA1UECgwLRXhhbXBs
ZSBPcmcxCzAJBgNVBAYTAlVTMB4XDTI2MDYwODIxMTcwOFoXDTM2MDYwNTIxMTcw
OFowPjEZMBcGA1UEAwwQdGVzdC5leGFtcGxlLmNvbTEUMBIGA1UECgwLRXhhbXBs
ZSBPcmcxCzAJBgNVBAYTAlVTMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKC
AQEAqKDmxDlyeKLjdc6OQ9jVeoSaJ2MYO171WL9yMT4ay6klC/sXrrAmoito+3/U
Gke7CR+zAD+Tww24GNKSLBwNdsYQLhrg8AL9MDNg11Xj06uxd3LRDMZPKcoQGiz2
ihxoxugqIWlDzLncVTcqmRTgCHgpSZ+vcT8CRVvAbHbBF1WehGHTMeY8+kluENOL
Aqx7SDUGbgtblnLyAtN+l6TTfbvyyqC8LAT5/dfo8eyW/DxKClaxbbQTYV8pbI7z
rjujV4CocCCZ+HKKzEOMYO0ILOx8+xZrRYnD8GFjdYuqVrOs8oJZl7qZJ/KMqAUy
5YbFawlBvo5hJpFmTt1ieMSnWQIDAQABo4GwMIGtMB0GA1UdDgQWBBSgNqzeuWfy
b6Ocy2GDFGY1sJdg5TAfBgNVHSMEGDAWgBSgNqzeuWfyb6Ocy2GDFGY1sJdg5TAP
BgNVHRMBAf8EBTADAQH/MFoGA1UdEQRTMFGCEHRlc3QuZXhhbXBsZS5jb22CD3d3
dy5leGFtcGxlLmNvbYcEwKgBCoERYWRtaW5AZXhhbXBsZS5jb22GE2h0dHBzOi8v
ZXhhbXBsZS5jb20wDQYJKoZIhvcNAQELBQADggEBAHmZEtPX7XphRmwvB49l8j4P
k7PImlatc+3RPEsV5ikpQb4/tMer/QKOptGjxMxnBGIcXRz0DWv3azHOzGxAO9Rb
MO57XPze3W//JxBZqkqkFKYjOW7vsq2R5UXxhrdNCVYjkhIFOydNjdX42qndjDOl
N6c0PpBLRVFFa97AeDdb/uOBFyeCiqMqeBeRSXRJ2WLCAAOqiRo72ygzD5f+/GmM
SYQh6k2cA1HzZx9mp7gGeRL9GeFyLHE55RXkSJuDsRObV2TpZ7kDWdgG910bXu1f
9ZjZrhDEhnoG2TYhkpFb/ZIYPx6kBlxUBsfJpq5YDzCi+06gvwBlj57bP1OxT6Q=
-----END CERTIFICATE-----
"#;

#[tokio::test]
async fn test_fetch_certificate_by_id() {
    let mut app = TestApp::new().await;
    let response = app
        .create_new_certificate(&serde_json::json!({ "pem_data": VALID_CERT_PEM }))
        .await;
    assert_eq!(response.status().as_u16(), 201);

    let body: serde_json::Value = response.json().await.unwrap();
    let id = body["id"].as_str().unwrap();

    let response_2 = app.fetch_certificate_by_id(id).await;
    assert_eq!(response_2.status().as_u16(), 200);

    app.clean_up().await;
}
