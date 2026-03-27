use std::collections::HashMap;

use crate::domain::{
    Email, {LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let _ignore_old_value = self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes
            .remove(email)
            .map(|_| ())
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        self.codes
            .get(email)
            .cloned()
            .ok_or(TwoFACodeStoreError::LoginAttemptIdNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::SecretString;
    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapTwoFACodeStore::default();

        let email =
            Email::parse(SecretString::new("vas@email".to_owned().into_boxed_str())).unwrap();
        let loginid = LoginAttemptId::default();
        let code = TwoFACode::default();
        assert_eq!(
            store
                .add_code(email.clone(), loginid.clone(), code.clone())
                .await,
            Ok(())
        );
        assert_eq!(
            store
                .add_code(email.clone(), loginid.clone(), code.clone())
                .await,
            Ok(())
        );

        let (logidresult, coderesult) = store.get_code(&email).await.unwrap();
        assert_eq!(loginid, logidresult);
        assert_eq!(code, coderesult);
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapTwoFACodeStore::default();

        let email =
            Email::parse(SecretString::new("vas@email".to_owned().into_boxed_str())).unwrap();
        let loginid = LoginAttemptId::default();
        let code = TwoFACode::default();
        store
            .add_code(email.clone(), loginid.clone(), code.clone())
            .await
            .unwrap();

        let (logidresult, coderesult) = store.get_code(&email).await.unwrap();
        assert_eq!(loginid, logidresult);
        assert_eq!(code, coderesult);

        let email = Email::parse(SecretString::new(
            "emailwasnot@added".to_owned().into_boxed_str(),
        ))
        .unwrap();

        assert!(store.get_code(&email).await.is_err());
    }

    #[tokio::test]
    async fn test_remove_user() {
        let mut store = HashmapTwoFACodeStore::default();

        let email =
            Email::parse(SecretString::new("vas@email".to_owned().into_boxed_str())).unwrap();
        let loginid = LoginAttemptId::default();
        let code = TwoFACode::default();
        let _ = store
            .add_code(email.clone(), loginid.clone(), code.clone())
            .await;

        let (logidresult, coderesult) = store.get_code(&email).await.unwrap();
        assert_eq!(loginid, logidresult);
        assert_eq!(code, coderesult);

        assert!(store.remove_code(&email).await.is_ok());
        assert!(store.remove_code(&email).await.is_err());

        assert!(store
            .remove_code(
                &Email::parse(SecretString::new(
                    "emailwasnot@added".to_owned().into_boxed_str()
                ))
                .unwrap()
            )
            .await
            .is_err());
    }
}
