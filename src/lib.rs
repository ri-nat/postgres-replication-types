use chrono::{DateTime, Utc};

/// A logical replication message.
pub enum ReplicationMessage<'a> {
    Begin(BeginMessage),
    Generic(GenericMessage<'a>),
    Commit(CommitMessage),
    Origin(OriginMessage),
    Relation(RelationMessage),
    Type(TypeMessage),
    Insert(InsertMessage<'a>),
    Update(UpdateMessage<'a>),
    Delete(DeleteMessage<'a>),
    Truncate(TruncateMessage),
    StreamStart(StreamStartMessage),
    StreamStop(StreamStopMessage),
    StreamCommit(StreamCommitMessage),
    StreamAbort(StreamAbortMessage),
    BeginPrepare(BeginPrepareMessage),
    Prepare(PrepareMessage),
    CommitPrepared(CommitPreparedMessage),
    RollbackPrepared(RollbackPreparedMessage),
    StreamPrepare(StreamPrepareMessage),
}

pub struct BeginMessage {
    /// The final LSN of the transaction.
    pub final_lsn: i64,
    /// Commit timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
    /// Xid of the transaction.
    pub transaction_id: i32,
}

pub struct GenericMessage<'a> {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// If the logical decoding message is transactional?
    pub is_transactional: bool,
    /// The LSN of the logical decoding message.
    pub lsn: i64,
    /// The prefix of the logical decoding message.
    pub prefix: String,
    /// Length of the content.
    pub length: i32,
    /// The content of the logical decoding message.
    pub content: &'a [u8],
}

pub struct CommitMessage {
    /// The LSN of the commit.
    pub lsn: i64,
    /// The final LSN of the transaction.
    pub final_lsn: i64,
    /// Commit timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
}

pub struct OriginMessage {
    /// The LSN of the commit on the origin server.
    pub lsn: i64,
    /// Name of the origin.
    ///
    /// NOTE: There can be multiple Origin messages inside a single transaction.
    pub name: String,
}

pub struct RelationMessage {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// OID of the relation.
    pub oid: i32,
    /// Namespace (`None` for `pg_catalog`).
    pub namespace: Option<String>,
    /// Relation name.
    pub name: String,
    /// Replica identity setting for the relation (same as `relreplident` in `pg_class`).
    pub replica_identity: i8,
    /// Columns.
    pub columns: Vec<RelationMessageColumn>,
}

pub struct RelationMessageColumn {
    /// Is part of the key?
    pub is_part_of_the_key: bool,
    /// Name of the column.
    pub name: String,
    /// OID of the column's data type.
    pub oid: i32,
    /// Type modifier of the column (`atttypmod`).
    pub type_modifier: i32,
}

pub struct TypeMessage {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// OID of the relation.
    pub oid: i32,
    /// Namespace (`None` for `pg_catalog`).
    pub namespace: Option<String>,
    /// Name of the data type.
    pub name: String,
}

pub struct InsertMessage<'a> {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// OID of the relation.
    pub oid: i32,
    /// [`TupleData`] message part representing the contents of new tuple.
    pub data: TupleData<'a>,
}

pub struct UpdateMessage<'a> {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// OID of the relation corresponding to the ID in the relation message.
    pub oid: i32,
    /// This field is optional and is only present if the update changed data in any of the column(s) that are part of the REPLICA IDENTITY index.
    pub key: Option<TupleData<'a>>,
    /// This field is optional and is only present if table in which the update happened has REPLICA IDENTITY set to FULL.
    pub old: Option<TupleData<'a>>,
    /// TupleData message part representing the contents of a new tuple.
    pub new: TupleData<'a>,
}

pub struct DeleteMessage<'a> {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// OID of the relation corresponding to the ID in the relation message.
    pub oid: i32,
    /// This field is optional and is only present if the update changed data in any of the column(s) that are part of the REPLICA IDENTITY index.
    pub key: Option<TupleData<'a>>,
    /// This field is optional and is only present if table in which the update happened has REPLICA IDENTITY set to FULL.
    pub old: Option<TupleData<'a>>,
}

pub struct TruncateMessage {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// Number of relations
    pub relations_count: i32,
    /// Is `CASCADE`?
    pub is_cascade: bool,
    /// Is `RESTART IDENTITY`?
    pub is_restart_identity: bool,
    /// OID of the relation corresponding to the ID in the relation message.
    pub oid: i32,
}

pub struct StreamStartMessage {
    /// Xid of the transaction (only present for streamed transactions).
    ///
    /// NOTE: This field is available since protocol version 2.
    pub transaction_id: Option<i32>,
    /// Is it a first stream segment?
    pub is_first_segment: bool,
}

pub struct StreamStopMessage {}

pub struct StreamCommitMessage {
    /// Xid of the transaction.
    pub transaction_id: i32,
    /// The LSN of the commit.
    pub lsn: i64,
    /// The end LSN of the transaction.
    pub final_lsn: i64,
    /// Commit timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
}

pub struct StreamAbortMessage {
    /// Xid of the transaction.
    pub transaction_id: i32,
    /// Xid of the subtransaction (will be same as xid of the transaction for top-level transactions).
    pub subtransaction_id: i32,
}

pub struct BeginPrepareMessage {
    /// The LSN of the prepare.
    pub lsn: i64,
    /// The end LSN of the prepared transaction.
    pub final_lsn: i64,
    /// Prepare timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
    /// Xid of the transaction.
    pub transaction_id: i32,
    /// The user defined GID of the prepared transaction.
    pub gid: String,
}

pub struct PrepareMessage {
    /// The LSN of the prepare.
    pub lsn: i64,
    /// The end LSN of the prepared transaction.
    pub final_lsn: i64,
    /// Prepare timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
    /// Xid of the transaction.
    pub transaction_id: i32,
    /// The user defined GID of the prepared transaction.
    pub gid: String,
}

pub struct CommitPreparedMessage {
    /// The LSN of the commit.
    pub lsn: i64,
    /// The end LSN of the prepared transaction.
    pub final_lsn: i64,
    /// Commit timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
    /// Xid of the transaction.
    pub transaction_id: i32,
    /// The user defined GID of the prepared transaction.
    pub gid: String,
}

pub struct RollbackPreparedMessage {
    /// The LSN of the rollback.
    pub lsn: i64,
    /// The end LSN of the rollback or the prepared transaction.
    pub final_lsn: i64,
    /// Prepare timestamp of the transaction.
    pub prepare_timestamp: DateTime<Utc>,
    /// Rollback timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
    /// Xid of the transaction.
    pub transaction_id: i32,
    /// The user defined GID of the prepared transaction.
    pub gid: String,
}

pub struct StreamPrepareMessage {
    /// The LSN of the prepare.
    pub lsn: i64,
    /// The end LSN of the prepared transaction.
    pub final_lsn: i64,
    /// Prepare timestamp of the transaction.
    pub timestamp: DateTime<Utc>,
    /// Xid of the transaction.
    pub transaction_id: i32,
    /// The user defined GID of the prepared transaction.
    pub gid: String,
}

pub struct TupleData<'a> {
    /// Columns.
    pub columns: Vec<TupleDataColumn<'a>>,
}

pub struct TupleDataColumn<'a> {
    /// Identifies the data as NULL value.
    pub is_null: bool,
    /// Identifies unchanged TOASTed value (the actual value is not sent).
    /// TODO: decide correct naming here after research
    pub is_unchanged: bool,
    /// Identifies the data as text formatted value.
    pub is_text: bool,
    /// Identifies the data as binary formatted value.
    pub is_binary: bool,
    /// The value of the column in bytes. Only present if `is_binary` is `true`.
    pub binary_value: Option<&'a [u8]>,
    /// The value of the column as [`String`]. Only present if `is_text` is `true`,
    pub text_value: Option<String>,
}
