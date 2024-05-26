use crate::types::MadatoError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum MadatoCalError {
    #[error("Data Row is Empty")]
    MissingDataInSheet(),

    #[error("Calamine Error: IO error: {0}")]
    IoError(String),

    #[error("Calamine Error: Ods error: {0}")]
    OdsError(String),

    #[error("Calamine Error: Xls error: {0}")]
    XlsError(String),

    #[error("Calamine Error: Xlsb error: {0}")]
    XlsbError(String),

    #[error("Calamine Error: Xlsx error: {0}")]
    XlsxError(String),

    #[error("Calamine Error: Vba error: {0}")]
    VbaError(String),

    #[error("Calamine Error: Deserialization error: {0}")]
    DeError(String),

    #[error("Calamine Error: General error: {0}")]
    GeneralError(String),
}

impl From<calamine::Error> for MadatoCalError {
    fn from(e: calamine::Error) -> Self {
        match e {
            calamine::Error::Io(err) => MadatoCalError::IoError(err.to_string()),
            calamine::Error::Ods(err) => MadatoCalError::OdsError(err.to_string()),
            calamine::Error::Xls(err) => MadatoCalError::XlsError(err.to_string()),
            calamine::Error::Xlsb(err) => MadatoCalError::XlsbError(err.to_string()),
            calamine::Error::Xlsx(err) => MadatoCalError::XlsxError(err.to_string()),
            calamine::Error::Vba(err) => MadatoCalError::VbaError(err.to_string()),
            calamine::Error::De(err) => MadatoCalError::DeError(err.to_string()),
            calamine::Error::Msg(msg) => MadatoCalError::GeneralError(msg.to_string()),
        }
    }
}

impl From<MadatoCalError> for MadatoError {
    fn from(e: MadatoCalError) -> Self {
        MadatoError::CalError(e.to_string())
    }
}
