#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type EmailDataProviderConnection = *mut ::core::ffi::c_void;
pub type EmailDataProviderTriggerDetails = *mut ::core::ffi::c_void;
pub type EmailMailboxCreateFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxCreateFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxDeleteFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxDeleteFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadAttachmentRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadAttachmentRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadMessageRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxDownloadMessageRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxEmptyFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxEmptyFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxForwardMeetingRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxForwardMeetingRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxGetAutoReplySettingsRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxGetAutoReplySettingsRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxMoveFolderRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxMoveFolderRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxProposeNewTimeForMeetingRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxProposeNewTimeForMeetingRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxResolveRecipientsRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxResolveRecipientsRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxServerSearchReadBatchRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxServerSearchReadBatchRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxSetAutoReplySettingsRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxSetAutoReplySettingsRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxSyncManagerSyncRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxSyncManagerSyncRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxUpdateMeetingResponseRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxUpdateMeetingResponseRequestEventArgs = *mut ::core::ffi::c_void;
pub type EmailMailboxValidateCertificatesRequest = *mut ::core::ffi::c_void;
pub type EmailMailboxValidateCertificatesRequestEventArgs = *mut ::core::ffi::c_void;
