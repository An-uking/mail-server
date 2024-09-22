/*
 * SPDX-FileCopyrightText: 2020 Stalwart Labs Ltd <hello@stalw.art>
 *
 * SPDX-License-Identifier: AGPL-3.0-only OR LicenseRef-SEL
 */

use crate::Permission;

pub mod cache;
pub mod config;
pub mod dispatch;
pub mod principal;
pub mod secret;

impl Permission {
    pub fn description(&self) -> &'static str {
        match self {
            Permission::Impersonate => "Allows acting on behalf of another user",
            Permission::UnlimitedRequests => "Removes request limits or quotas",
            Permission::UnlimitedUploads => "Removes upload size or frequency limits",
            Permission::DeleteSystemFolders => "Allows deletion of critical system folders",
            Permission::MessageQueueList => "View message queue",
            Permission::MessageQueueGet => "Retrieve specific messages from the queue",
            Permission::MessageQueueUpdate => "Modify queued messages",
            Permission::MessageQueueDelete => "Remove messages from the queue",
            Permission::OutgoingReportList => "View reports for outgoing emails",
            Permission::OutgoingReportGet => "Retrieve specific outgoing email reports",
            Permission::OutgoingReportDelete => "Remove outgoing email reports",
            Permission::IncomingReportList => "View reports for incoming emails",
            Permission::IncomingReportGet => "Retrieve specific incoming email reports",
            Permission::IncomingReportDelete => "Remove incoming email reports",
            Permission::SettingsList => "View system settings",
            Permission::SettingsUpdate => "Modify system settings",
            Permission::SettingsDelete => "Remove system settings",
            Permission::SettingsReload => "Refresh system settings",
            Permission::IndividualList => "View list of individual users",
            Permission::IndividualGet => "Retrieve specific user information",
            Permission::IndividualUpdate => "Modify user information",
            Permission::IndividualDelete => "Remove user accounts",
            Permission::IndividualCreate => "Add new user accounts",
            Permission::GroupList => "View list of user groups",
            Permission::GroupGet => "Retrieve specific group information",
            Permission::GroupUpdate => "Modify group information",
            Permission::GroupDelete => "Remove user groups",
            Permission::GroupCreate => "Add new user groups",
            Permission::DomainList => "View list of email domains",
            Permission::DomainGet => "Retrieve specific domain information",
            Permission::DomainCreate => "Add new email domains",
            Permission::DomainUpdate => "Modify domain information",
            Permission::DomainDelete => "Remove email domains",
            Permission::TenantList => "View list of tenants (in multi-tenant setup)",
            Permission::TenantGet => "Retrieve specific tenant information",
            Permission::TenantCreate => "Add new tenants",
            Permission::TenantUpdate => "Modify tenant information",
            Permission::TenantDelete => "Remove tenants",
            Permission::MailingListList => "View list of mailing lists",
            Permission::MailingListGet => "Retrieve specific mailing list information",
            Permission::MailingListCreate => "Create new mailing lists",
            Permission::MailingListUpdate => "Modify mailing list information",
            Permission::MailingListDelete => "Remove mailing lists",
            Permission::RoleList => "View list of roles",
            Permission::RoleGet => "Retrieve specific role information",
            Permission::RoleCreate => "Create new roles",
            Permission::RoleUpdate => "Modify role information",
            Permission::RoleDelete => "Remove roles",
            Permission::PrincipalList => "View list of principals (users or system entities)",
            Permission::PrincipalGet => "Retrieve specific principal information",
            Permission::PrincipalCreate => "Create new principals",
            Permission::PrincipalUpdate => "Modify principal information",
            Permission::PrincipalDelete => "Remove principals",
            Permission::BlobFetch => "Retrieve binary large objects",
            Permission::PurgeBlobStore => "Clear the blob storage",
            Permission::PurgeDataStore => "Clear the data storage",
            Permission::PurgeLookupStore => "Clear the lookup storage",
            Permission::PurgeAccount => "Completely remove an account and all associated data",
            Permission::FtsReindex => "Rebuild the full-text search index",
            Permission::Undelete => "Restore deleted items",
            Permission::DkimSignatureCreate => "Create DKIM signatures for email authentication",
            Permission::DkimSignatureGet => "Retrieve DKIM signature information",
            Permission::UpdateSpamFilter => "Modify spam filter settings",
            Permission::UpdateWebadmin => "Modify web admin interface settings",
            Permission::LogsView => "Access system logs",
            Permission::SieveRun => "Execute Sieve scripts for email filtering",
            Permission::Restart => "Restart the email server",
            Permission::TracingList => "View list of system traces",
            Permission::TracingGet => "Retrieve specific trace information",
            Permission::TracingLive => "View real-time system traces",
            Permission::MetricsList => "View list of system metrics",
            Permission::MetricsLive => "View real-time system metrics",
            Permission::Authenticate => "Perform authentication",
            Permission::AuthenticateOauth => "Perform OAuth authentication",
            Permission::EmailSend => "Send emails",
            Permission::EmailReceive => "Receive emails",
            Permission::ManageEncryption => "Handle encryption settings and operations",
            Permission::ManagePasswords => "Manage user passwords",
            Permission::JmapEmailGet => "Retrieve emails via JMAP",
            Permission::JmapMailboxGet => "Retrieve mailboxes via JMAP",
            Permission::JmapThreadGet => "Retrieve email threads via JMAP",
            Permission::JmapIdentityGet => "Retrieve user identities via JMAP",
            Permission::JmapEmailSubmissionGet => "Retrieve email submission info via JMAP",
            Permission::JmapPushSubscriptionGet => "Retrieve push subscriptions via JMAP",
            Permission::JmapSieveScriptGet => "Retrieve Sieve scripts via JMAP",
            Permission::JmapVacationResponseGet => "Retrieve vacation responses via JMAP",
            Permission::JmapPrincipalGet => "Retrieve principal information via JMAP",
            Permission::JmapQuotaGet => "Retrieve quota information via JMAP",
            Permission::JmapBlobGet => "Retrieve blobs via JMAP",
            Permission::JmapEmailSet => "Modify emails via JMAP",
            Permission::JmapMailboxSet => "Modify mailboxes via JMAP",
            Permission::JmapIdentitySet => "Modify user identities via JMAP",
            Permission::JmapEmailSubmissionSet => "Modify email submission settings via JMAP",
            Permission::JmapPushSubscriptionSet => "Modify push subscriptions via JMAP",
            Permission::JmapSieveScriptSet => "Modify Sieve scripts via JMAP",
            Permission::JmapVacationResponseSet => "Modify vacation responses via JMAP",
            Permission::JmapEmailChanges => "Track email changes via JMAP",
            Permission::JmapMailboxChanges => "Track mailbox changes via JMAP",
            Permission::JmapThreadChanges => "Track thread changes via JMAP",
            Permission::JmapIdentityChanges => "Track identity changes via JMAP",
            Permission::JmapEmailSubmissionChanges => "Track email submission changes via JMAP",
            Permission::JmapQuotaChanges => "Track quota changes via JMAP",
            Permission::JmapEmailCopy => "Copy emails via JMAP",
            Permission::JmapBlobCopy => "Copy blobs via JMAP",
            Permission::JmapEmailImport => "Import emails via JMAP",
            Permission::JmapEmailParse => "Parse emails via JMAP",
            Permission::JmapEmailQueryChanges => "Track email query changes via JMAP",
            Permission::JmapMailboxQueryChanges => "Track mailbox query changes via JMAP",
            Permission::JmapEmailSubmissionQueryChanges => {
                "Track email submission query changes via JMAP"
            }
            Permission::JmapSieveScriptQueryChanges => "Track Sieve script query changes via JMAP",
            Permission::JmapPrincipalQueryChanges => "Track principal query changes via JMAP",
            Permission::JmapQuotaQueryChanges => "Track quota query changes via JMAP",
            Permission::JmapEmailQuery => "Perform email queries via JMAP",
            Permission::JmapMailboxQuery => "Perform mailbox queries via JMAP",
            Permission::JmapEmailSubmissionQuery => "Perform email submission queries via JMAP",
            Permission::JmapSieveScriptQuery => "Perform Sieve script queries via JMAP",
            Permission::JmapPrincipalQuery => "Perform principal queries via JMAP",
            Permission::JmapQuotaQuery => "Perform quota queries via JMAP",
            Permission::JmapSearchSnippet => "Retrieve search snippets via JMAP",
            Permission::JmapSieveScriptValidate => "Validate Sieve scripts via JMAP",
            Permission::JmapBlobLookup => "Look up blobs via JMAP",
            Permission::JmapBlobUpload => "Upload blobs via JMAP",
            Permission::JmapEcho => "Perform JMAP echo requests",
            Permission::ImapAuthenticate => "Authenticate via IMAP",
            Permission::ImapAclGet => "Retrieve ACLs via IMAP",
            Permission::ImapAclSet => "Set ACLs via IMAP",
            Permission::ImapMyRights => "Retrieve own rights via IMAP",
            Permission::ImapListRights => "List rights via IMAP",
            Permission::ImapAppend => "Append messages via IMAP",
            Permission::ImapCapability => "Retrieve server capabilities via IMAP",
            Permission::ImapId => "Retrieve server ID via IMAP",
            Permission::ImapCopy => "Copy messages via IMAP",
            Permission::ImapMove => "Move messages via IMAP",
            Permission::ImapCreate => "Create mailboxes via IMAP",
            Permission::ImapDelete => "Delete mailboxes or messages via IMAP",
            Permission::ImapEnable => "Enable IMAP extensions",
            Permission::ImapExpunge => "Expunge deleted messages via IMAP",
            Permission::ImapFetch => "Fetch messages or metadata via IMAP",
            Permission::ImapIdle => "Use IMAP IDLE command",
            Permission::ImapList => "List mailboxes via IMAP",
            Permission::ImapLsub => "List subscribed mailboxes via IMAP",
            Permission::ImapNamespace => "Retrieve namespaces via IMAP",
            Permission::ImapRename => "Rename mailboxes via IMAP",
            Permission::ImapSearch => "Search messages via IMAP",
            Permission::ImapSort => "Sort messages via IMAP",
            Permission::ImapSelect => "Select mailboxes via IMAP",
            Permission::ImapExamine => "Examine mailboxes via IMAP",
            Permission::ImapStatus => "Retrieve mailbox status via IMAP",
            Permission::ImapStore => "Modify message flags via IMAP",
            Permission::ImapSubscribe => "Subscribe to mailboxes via IMAP",
            Permission::ImapThread => "Thread messages via IMAP",
            Permission::Pop3Authenticate => "Authenticate via POP3",
            Permission::Pop3List => "List messages via POP3",
            Permission::Pop3Uidl => "Retrieve unique IDs via POP3",
            Permission::Pop3Stat => "Retrieve mailbox statistics via POP3",
            Permission::Pop3Retr => "Retrieve messages via POP3",
            Permission::Pop3Dele => "Mark messages for deletion via POP3",
            Permission::SieveAuthenticate => "Authenticate for Sieve script management",
            Permission::SieveListScripts => "List Sieve scripts",
            Permission::SieveSetActive => "Set active Sieve script",
            Permission::SieveGetScript => "Retrieve Sieve scripts",
            Permission::SievePutScript => "Upload Sieve scripts",
            Permission::SieveDeleteScript => "Delete Sieve scripts",
            Permission::SieveRenameScript => "Rename Sieve scripts",
            Permission::SieveCheckScript => "Validate Sieve scripts",
            Permission::SieveHaveSpace => "Check available space for Sieve scripts",
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Permission;

    #[test]
    #[ignore]
    fn print_permissions() {
        const CHECK: &str = ":white_check_mark:";

        for permission in Permission::all() {
            println!(
                "|`{}`|{}|{}|{}|{}|",
                permission.name(),
                permission.description(),
                CHECK,
                permission
                    .is_tenant_admin_permission()
                    .then_some(CHECK)
                    .unwrap_or_default(),
                permission
                    .is_user_permission()
                    .then_some(CHECK)
                    .unwrap_or_default()
            );
        }
    }
}
