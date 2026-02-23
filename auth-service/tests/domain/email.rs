
    use auth_service::domain ::Email;
    #[test]
    fn check_email() {
        assert!(Email::parse("vas@gmial.com".to_owned()).is_ok());
        assert!(Email::parse("@gmial.com".to_owned()).is_err());
        assert!(Email::parse("".to_owned()).is_err());
        // let email = Email("".to_owned());
        // assert!(Email("".()).is_err());

    }
