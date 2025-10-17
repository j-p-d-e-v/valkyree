use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug)]
pub struct Auth {}
impl Auth {
    pub fn build(config: &AuthConfig) -> anyhow::Result<String> {
        let mut value = "AUTH".to_string();
        if config.username.is_some() && config.password.is_none() {
            return Err(anyhow::anyhow!("USERNAME_NO_PASSWORD".to_string()));
        }
        if let Some(username) = &config.username {
            if username.is_empty() {
                return Err(anyhow!("USERNAME_REQUIRED".to_string()));
            }
            value = format!("{value} {username}");
        }
        if let Some(password) = &config.password {
            if password.is_empty() {
                return Err(anyhow!("PASSWORD_EMPTY".to_string()));
            }
            value = format!("{value} {password}");
        } else {
            return Err(anyhow!("PASSWORD_REQUIRED".to_string()));
        }
        Ok(format!("{value}\r\n"))
    }
}

#[cfg(test)]
pub mod test_auth {
    use super::*;

    #[test]
    fn test_password_only() {
        let result = Auth::build(&AuthConfig {
            username: None,
            password: Some("mypassword".to_string()),
        });
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!("AUTH mypassword\r\n".to_string(), result.unwrap());
    }

    #[test]
    fn test_username_password() {
        let result = Auth::build(&AuthConfig {
            username: Some("myusername".to_string()),
            password: Some("mypassword".to_string()),
        });
        assert!(result.is_ok(), "{:#?}", result.err());
        assert_eq!(
            "AUTH myusername mypassword\r\n".to_string(),
            result.unwrap()
        );
    }
    #[test]
    fn test_error_no_config() {
        let result = Auth::build(&AuthConfig {
            username: None,
            password: None,
        });
        assert!(result.is_err());
    }
    #[test]
    fn test_error_username_only() {
        let result = Auth::build(&AuthConfig {
            username: Some("myusername".to_string()),
            password: None,
        });
        assert!(result.is_err());
    }
}
