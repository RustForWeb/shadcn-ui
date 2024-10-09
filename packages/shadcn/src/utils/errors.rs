#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorType {
    MissingDirOrEmptyProject,
    ExistingConfig,
    MissingConfig,
    FailedConfigRead,
    TailwindNotConfigured,
    ImportAliasMissing,
    UnsupportedFramework,
    ComponentUrlNotFound,
    ComponentUrlUnauthorized,
    ComponentUrlForbidden,
    ComponentUrlBadRequest,
    ComponentUrlInternalServerError,
}
