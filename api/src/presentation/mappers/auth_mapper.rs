use crate::application::auth::commands::change_password::ChangePassword;
use crate::application::auth::commands::forgot_password::ForgotPassword;
use crate::application::auth::commands::login::Login;
use crate::application::auth::commands::register::Register;
use crate::application::auth::commands::reset_password::ResetPassword;
use crate::presentation::requests::auth::change_password_request::ChangePasswordRequest;
use crate::presentation::requests::auth::forgot_password_request::ForgotPasswordRequest;
use crate::presentation::requests::auth::login_request::LoginRequest;
use crate::presentation::requests::auth::register_request::RegisterRequest;
use crate::presentation::requests::auth::reset_password_request::ResetPasswordRequest;

impl From<ChangePasswordRequest> for ChangePassword {
    fn from(request: ChangePasswordRequest) -> Self {
        Self {
            old_password: request.old_password,
            new_password: request.new_password,
        }
    }
}

impl From<ForgotPasswordRequest> for ForgotPassword {
    fn from(request: ForgotPasswordRequest) -> Self {
        Self {
            email: request.email,
        }
    }
}

impl From<LoginRequest> for Login {
    fn from(request: LoginRequest) -> Self {
        Self {
            email: request.email,
            password: request.password,
        }
    }
}

impl From<RegisterRequest> for Register {
    fn from(request: RegisterRequest) -> Self {
        Self {
            email: request.email,
            username: request.username,
            password: request.password,
            full_name: request.full_name,
        }
    }
}

impl From<ResetPasswordRequest> for ResetPassword {
    fn from(request: ResetPasswordRequest) -> Self {
        Self {
            token: request.token,
            password: request.password,
        }
    }
}
