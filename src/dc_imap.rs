use c2rust_bitfields::BitfieldStruct;
use libc;
extern "C" {
    pub type mailstream_cancel;
    pub type sqlite3;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn atol(_: *const libc::c_char) -> libc::c_long;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn time(_: *mut time_t) -> time_t;
    #[no_mangle]
    fn mailstream_close(s: *mut mailstream) -> libc::c_int;
    #[no_mangle]
    fn mailstream_wait_idle(s: *mut mailstream, max_idle_delay: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn mailstream_setup_idle(s: *mut mailstream) -> libc::c_int;
    #[no_mangle]
    fn mailstream_unsetup_idle(s: *mut mailstream);
    #[no_mangle]
    fn mailstream_interrupt_idle(s: *mut mailstream);
    #[no_mangle]
    fn mailimap_section_new(sec_spec: *mut mailimap_section_spec) -> *mut mailimap_section;
    #[no_mangle]
    fn mailimap_set_free(set: *mut mailimap_set);
    #[no_mangle]
    fn mailimap_fetch_type_free(fetch_type: *mut mailimap_fetch_type);
    #[no_mangle]
    fn mailimap_store_att_flags_free(store_att_flags: *mut mailimap_store_att_flags);
    #[no_mangle]
    fn mailimap_set_new_interval(first: uint32_t, last: uint32_t) -> *mut mailimap_set;
    #[no_mangle]
    fn mailimap_set_new_single(indx: uint32_t) -> *mut mailimap_set;
    #[no_mangle]
    fn mailimap_fetch_att_new_envelope() -> *mut mailimap_fetch_att;
    #[no_mangle]
    fn mailimap_fetch_att_new_flags() -> *mut mailimap_fetch_att;
    #[no_mangle]
    fn mailimap_fetch_att_new_uid() -> *mut mailimap_fetch_att;
    #[no_mangle]
    fn mailimap_fetch_att_new_body_peek_section(
        section: *mut mailimap_section,
    ) -> *mut mailimap_fetch_att;
    #[no_mangle]
    fn mailimap_fetch_type_new_fetch_att_list_empty() -> *mut mailimap_fetch_type;
    #[no_mangle]
    fn mailimap_fetch_type_new_fetch_att_list_add(
        fetch_type: *mut mailimap_fetch_type,
        fetch_att: *mut mailimap_fetch_att,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_store_att_flags_new_add_flags(
        flags: *mut mailimap_flag_list,
    ) -> *mut mailimap_store_att_flags;
    #[no_mangle]
    fn mailimap_flag_list_new_empty() -> *mut mailimap_flag_list;
    #[no_mangle]
    fn mailimap_flag_list_add(
        flag_list: *mut mailimap_flag_list,
        f: *mut mailimap_flag,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_flag_new_deleted() -> *mut mailimap_flag;
    #[no_mangle]
    fn mailimap_flag_new_seen() -> *mut mailimap_flag;
    #[no_mangle]
    fn mailimap_flag_new_flag_keyword(flag_keyword: *mut libc::c_char) -> *mut mailimap_flag;
    #[no_mangle]
    fn mailimap_socket_connect(
        f: *mut mailimap,
        server: *const libc::c_char,
        port: uint16_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_socket_starttls(f: *mut mailimap) -> libc::c_int;
    #[no_mangle]
    fn mailimap_ssl_connect(
        f: *mut mailimap,
        server: *const libc::c_char,
        port: uint16_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_uidplus_uid_copy(
        session: *mut mailimap,
        set: *mut mailimap_set,
        mb: *const libc::c_char,
        uidvalidity_result: *mut uint32_t,
        source_result: *mut *mut mailimap_set,
        dest_result: *mut *mut mailimap_set,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_uidplus_uid_move(
        session: *mut mailimap,
        set: *mut mailimap_set,
        mb: *const libc::c_char,
        uidvalidity_result: *mut uint32_t,
        source_result: *mut *mut mailimap_set,
        dest_result: *mut *mut mailimap_set,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_idle(session: *mut mailimap) -> libc::c_int;
    #[no_mangle]
    fn mailimap_idle_done(session: *mut mailimap) -> libc::c_int;
    #[no_mangle]
    fn mailimap_has_idle(session: *mut mailimap) -> libc::c_int;
    #[no_mangle]
    fn mailimap_has_xlist(session: *mut mailimap) -> libc::c_int;
    #[no_mangle]
    fn mailimap_oauth2_authenticate(
        session: *mut mailimap,
        auth_user: *const libc::c_char,
        access_token: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_close(session: *mut mailimap) -> libc::c_int;
    #[no_mangle]
    fn mailimap_fetch(
        session: *mut mailimap,
        set: *mut mailimap_set,
        fetch_type: *mut mailimap_fetch_type,
        result: *mut *mut clist,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_uid_fetch(
        session: *mut mailimap,
        set: *mut mailimap_set,
        fetch_type: *mut mailimap_fetch_type,
        result: *mut *mut clist,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_fetch_list_free(fetch_list: *mut clist);
    #[no_mangle]
    fn mailimap_login(
        session: *mut mailimap,
        userid: *const libc::c_char,
        password: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_select(session: *mut mailimap, mb: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn mailimap_uid_store(
        session: *mut mailimap,
        set: *mut mailimap_set,
        store_att_flags: *mut mailimap_store_att_flags,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimap_new(
        imap_progr_rate: size_t,
        imap_progr_fun: Option<unsafe extern "C" fn(_: size_t, _: size_t) -> ()>,
    ) -> *mut mailimap;
    #[no_mangle]
    fn mailimap_free(session: *mut mailimap);
    #[no_mangle]
    fn mailimap_set_timeout(session: *mut mailimap, timeout: time_t);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(_: *mut pthread_cond_t, _: *const pthread_condattr_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_timedwait(
        _: *mut pthread_cond_t,
        _: *mut pthread_mutex_t,
        _: *const timespec,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(_: *mut pthread_mutex_t, _: *const pthread_mutexattr_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> libc::c_int;
    /* string tools */
    #[no_mangle]
    fn dc_strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn dc_trim(_: *mut libc::c_char);
    #[no_mangle]
    fn dc_mprintf(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn dc_strbuilder_init(_: *mut dc_strbuilder_t, init_bytes: libc::c_int);
    #[no_mangle]
    fn dc_strbuilder_cat(_: *mut dc_strbuilder_t, text: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn dc_strbuilder_catf(_: *mut dc_strbuilder_t, format: *const libc::c_char, _: ...);
    /* Replaces the first `%1$s` in the given String-ID by the given value.
    The result must be free()'d! */
    #[no_mangle]
    fn dc_stock_str_repl_string(
        _: *mut dc_context_t,
        id: libc::c_int,
        value: *const libc::c_char,
    ) -> *mut libc::c_char;
    /* Replaces the first `%1$s` and `%2$s` in the given String-ID by the two given strings.
    The result must be free()'d! */
    #[no_mangle]
    fn dc_stock_str_repl_string2(
        _: *mut dc_context_t,
        id: libc::c_int,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn dc_log_info(_: *mut dc_context_t, data1: libc::c_int, msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dc_log_event(
        _: *mut dc_context_t,
        event_code: libc::c_int,
        data1: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn dc_log_event_seq(
        _: *mut dc_context_t,
        event_code: libc::c_int,
        sequence_start: *mut libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    // the following function may block due http-requests;
    // must not be called from the main thread or by the ui!
    #[no_mangle]
    fn dc_get_oauth2_access_token(
        _: *mut dc_context_t,
        addr: *const libc::c_char,
        code: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn dc_log_warning(_: *mut dc_context_t, data1: libc::c_int, msg: *const libc::c_char, _: ...);
    #[no_mangle]
    fn dc_log_error(_: *mut dc_context_t, data1: libc::c_int, msg: *const libc::c_char, _: ...);
}
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: libc::c_long,
    pub __opaque: [libc::c_char; 8],
}
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type ssize_t = __darwin_ssize_t;
pub type time_t = __darwin_time_t;
pub type pthread_cond_t = __darwin_pthread_cond_t;
pub type pthread_condattr_t = __darwin_pthread_condattr_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct carray_s {
    pub array: *mut *mut libc::c_void,
    pub len: libc::c_uint,
    pub max: libc::c_uint,
}
pub type carray = carray_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mailstream {
    pub buffer_max_size: size_t,
    pub write_buffer: *mut libc::c_char,
    pub write_buffer_len: size_t,
    pub read_buffer: *mut libc::c_char,
    pub read_buffer_len: size_t,
    pub low: *mut mailstream_low,
    pub idle: *mut mailstream_cancel,
    pub idling: libc::c_int,
    pub logger: Option<
        unsafe extern "C" fn(
            _: *mut mailstream,
            _: libc::c_int,
            _: *const libc::c_char,
            _: size_t,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub logger_context: *mut libc::c_void,
}
pub type mailstream = _mailstream;
pub type mailstream_low = _mailstream_low;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _mailstream_low {
    pub data: *mut libc::c_void,
    pub driver: *mut mailstream_low_driver,
    pub privacy: libc::c_int,
    pub identifier: *mut libc::c_char,
    pub timeout: libc::c_ulong,
    pub logger: Option<
        unsafe extern "C" fn(
            _: *mut mailstream_low,
            _: libc::c_int,
            _: *const libc::c_char,
            _: size_t,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub logger_context: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailstream_low_driver {
    pub mailstream_read: Option<
        unsafe extern "C" fn(_: *mut mailstream_low, _: *mut libc::c_void, _: size_t) -> ssize_t,
    >,
    pub mailstream_write: Option<
        unsafe extern "C" fn(_: *mut mailstream_low, _: *const libc::c_void, _: size_t) -> ssize_t,
    >,
    pub mailstream_close: Option<unsafe extern "C" fn(_: *mut mailstream_low) -> libc::c_int>,
    pub mailstream_get_fd: Option<unsafe extern "C" fn(_: *mut mailstream_low) -> libc::c_int>,
    pub mailstream_free: Option<unsafe extern "C" fn(_: *mut mailstream_low) -> ()>,
    pub mailstream_cancel: Option<unsafe extern "C" fn(_: *mut mailstream_low) -> ()>,
    pub mailstream_get_cancel:
        Option<unsafe extern "C" fn(_: *mut mailstream_low) -> *mut mailstream_cancel>,
    pub mailstream_get_certificate_chain:
        Option<unsafe extern "C" fn(_: *mut mailstream_low) -> *mut carray>,
    pub mailstream_setup_idle: Option<unsafe extern "C" fn(_: *mut mailstream_low) -> libc::c_int>,
    pub mailstream_unsetup_idle:
        Option<unsafe extern "C" fn(_: *mut mailstream_low) -> libc::c_int>,
    pub mailstream_interrupt_idle:
        Option<unsafe extern "C" fn(_: *mut mailstream_low) -> libc::c_int>,
}
pub type progress_function = unsafe extern "C" fn(_: size_t, _: size_t) -> ();
pub type mailprogress_function =
    unsafe extern "C" fn(_: size_t, _: size_t, _: *mut libc::c_void) -> ();
pub type unnamed = libc::c_uint;
pub const MAILSTREAM_IDLE_CANCELLED: unnamed = 4;
pub const MAILSTREAM_IDLE_TIMEOUT: unnamed = 3;
pub const MAILSTREAM_IDLE_HASDATA: unnamed = 2;
pub const MAILSTREAM_IDLE_INTERRUPTED: unnamed = 1;
pub const MAILSTREAM_IDLE_ERROR: unnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _MMAPString {
    pub str_0: *mut libc::c_char,
    pub len: size_t,
    pub allocated_len: size_t,
    pub fd: libc::c_int,
    pub mmapped_size: size_t,
}
pub type MMAPString = _MMAPString;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clistcell_s {
    pub data: *mut libc::c_void,
    pub previous: *mut clistcell_s,
    pub next: *mut clistcell_s,
}
pub type clistcell = clistcell_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clist_s {
    pub first: *mut clistcell,
    pub last: *mut clistcell,
    pub count: libc::c_int,
}
pub type clist = clist_s;
pub type clistiter = clistcell;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailsmtp {
    pub stream: *mut mailstream,
    pub progr_rate: size_t,
    pub progr_fun: Option<unsafe extern "C" fn(_: size_t, _: size_t) -> ()>,
    pub response: *mut libc::c_char,
    pub line_buffer: *mut MMAPString,
    pub response_buffer: *mut MMAPString,
    pub esmtp: libc::c_int,
    pub auth: libc::c_int,
    pub smtp_sasl: unnamed_0,
    pub smtp_max_msg_size: size_t,
    pub smtp_progress_fun:
        Option<unsafe extern "C" fn(_: size_t, _: size_t, _: *mut libc::c_void) -> ()>,
    pub smtp_progress_context: *mut libc::c_void,
    pub response_code: libc::c_int,
    pub smtp_timeout: time_t,
    pub smtp_logger: Option<
        unsafe extern "C" fn(
            _: *mut mailsmtp,
            _: libc::c_int,
            _: *const libc::c_char,
            _: size_t,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub smtp_logger_context: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_0 {
    pub sasl_conn: *mut libc::c_void,
    pub sasl_server_fqdn: *const libc::c_char,
    pub sasl_login: *const libc::c_char,
    pub sasl_auth_name: *const libc::c_char,
    pub sasl_password: *const libc::c_char,
    pub sasl_realm: *const libc::c_char,
    pub sasl_secret: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body {
    pub bd_type: libc::c_int,
    pub bd_data: unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_1 {
    pub bd_body_1part: *mut mailimap_body_type_1part,
    pub bd_body_mpart: *mut mailimap_body_type_mpart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_type_mpart {
    pub bd_list: *mut clist,
    pub bd_media_subtype: *mut libc::c_char,
    pub bd_ext_mpart: *mut mailimap_body_ext_mpart,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_ext_mpart {
    pub bd_parameter: *mut mailimap_body_fld_param,
    pub bd_disposition: *mut mailimap_body_fld_dsp,
    pub bd_language: *mut mailimap_body_fld_lang,
    pub bd_loc: *mut libc::c_char,
    pub bd_extension_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_fld_lang {
    pub lg_type: libc::c_int,
    pub lg_data: unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    pub lg_single: *mut libc::c_char,
    pub lg_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_fld_dsp {
    pub dsp_type: *mut libc::c_char,
    pub dsp_attributes: *mut mailimap_body_fld_param,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_fld_param {
    pub pa_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_type_1part {
    pub bd_type: libc::c_int,
    pub bd_data: unnamed_3,
    pub bd_ext_1part: *mut mailimap_body_ext_1part,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_ext_1part {
    pub bd_md5: *mut libc::c_char,
    pub bd_disposition: *mut mailimap_body_fld_dsp,
    pub bd_language: *mut mailimap_body_fld_lang,
    pub bd_loc: *mut libc::c_char,
    pub bd_extension_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_3 {
    pub bd_type_basic: *mut mailimap_body_type_basic,
    pub bd_type_msg: *mut mailimap_body_type_msg,
    pub bd_type_text: *mut mailimap_body_type_text,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_type_text {
    pub bd_media_text: *mut libc::c_char,
    pub bd_fields: *mut mailimap_body_fields,
    pub bd_lines: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_fields {
    pub bd_parameter: *mut mailimap_body_fld_param,
    pub bd_id: *mut libc::c_char,
    pub bd_description: *mut libc::c_char,
    pub bd_encoding: *mut mailimap_body_fld_enc,
    pub bd_size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_fld_enc {
    pub enc_type: libc::c_int,
    pub enc_value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_type_msg {
    pub bd_fields: *mut mailimap_body_fields,
    pub bd_envelope: *mut mailimap_envelope,
    pub bd_body: *mut mailimap_body,
    pub bd_lines: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_envelope {
    pub env_date: *mut libc::c_char,
    pub env_subject: *mut libc::c_char,
    pub env_from: *mut mailimap_env_from,
    pub env_sender: *mut mailimap_env_sender,
    pub env_reply_to: *mut mailimap_env_reply_to,
    pub env_to: *mut mailimap_env_to,
    pub env_cc: *mut mailimap_env_cc,
    pub env_bcc: *mut mailimap_env_bcc,
    pub env_in_reply_to: *mut libc::c_char,
    pub env_message_id: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_env_bcc {
    pub bcc_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_env_cc {
    pub cc_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_env_to {
    pub to_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_env_reply_to {
    pub rt_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_env_sender {
    pub snd_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_env_from {
    pub frm_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_body_type_basic {
    pub bd_media_basic: *mut mailimap_media_basic,
    pub bd_fields: *mut mailimap_body_fields,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_media_basic {
    pub med_type: libc::c_int,
    pub med_basic_type: *mut libc::c_char,
    pub med_subtype: *mut libc::c_char,
}
pub type unnamed_4 = libc::c_uint;
pub const MAILIMAP_CAPABILITY_NAME: unnamed_4 = 1;
pub const MAILIMAP_CAPABILITY_AUTH_TYPE: unnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_capability {
    pub cap_type: libc::c_int,
    pub cap_data: unnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_5 {
    pub cap_auth_type: *mut libc::c_char,
    pub cap_name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_capability_data {
    pub cap_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_extension_data {
    pub ext_extension: *mut mailimap_extension_api,
    pub ext_type: libc::c_int,
    pub ext_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_extension_api {
    pub ext_name: *mut libc::c_char,
    pub ext_id: libc::c_int,
    pub ext_parser: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut mailstream,
            _: *mut MMAPString,
            _: *mut mailimap_parser_context,
            _: *mut size_t,
            _: *mut *mut mailimap_extension_data,
            _: size_t,
            _: Option<unsafe extern "C" fn(_: size_t, _: size_t) -> ()>,
        ) -> libc::c_int,
    >,
    pub ext_free: Option<unsafe extern "C" fn(_: *mut mailimap_extension_data) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_parser_context {
    pub is_rambler_workaround_enabled: libc::c_int,
    pub is_qip_workaround_enabled: libc::c_int,
    pub msg_body_handler: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut mailimap_msg_att_body_section,
            _: *const libc::c_char,
            _: size_t,
            _: *mut libc::c_void,
        ) -> bool,
    >,
    pub msg_body_handler_context: *mut libc::c_void,
    pub msg_body_section: *mut mailimap_msg_att_body_section,
    pub msg_body_att_type: libc::c_int,
    pub msg_body_parse_in_progress: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_msg_att_body_section {
    pub sec_section: *mut mailimap_section,
    pub sec_origin_octet: uint32_t,
    pub sec_body_part: *mut libc::c_char,
    pub sec_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_section {
    pub sec_spec: *mut mailimap_section_spec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_section_spec {
    pub sec_type: libc::c_int,
    pub sec_data: unnamed_6,
    pub sec_text: *mut mailimap_section_text,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_section_text {
    pub sec_type: libc::c_int,
    pub sec_msgtext: *mut mailimap_section_msgtext,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_section_msgtext {
    pub sec_type: libc::c_int,
    pub sec_header_list: *mut mailimap_header_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_header_list {
    pub hdr_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_6 {
    pub sec_msgtext: *mut mailimap_section_msgtext,
    pub sec_part: *mut mailimap_section_part,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_section_part {
    pub sec_id: *mut clist,
}
pub type mailimap_msg_body_handler = unsafe extern "C" fn(
    _: libc::c_int,
    _: *mut mailimap_msg_att_body_section,
    _: *const libc::c_char,
    _: size_t,
    _: *mut libc::c_void,
) -> bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_date_time {
    pub dt_day: libc::c_int,
    pub dt_month: libc::c_int,
    pub dt_year: libc::c_int,
    pub dt_hour: libc::c_int,
    pub dt_min: libc::c_int,
    pub dt_sec: libc::c_int,
    pub dt_zone: libc::c_int,
}
pub type unnamed_7 = libc::c_uint;
pub const MAILIMAP_FLAG_EXTENSION: unnamed_7 = 6;
pub const MAILIMAP_FLAG_KEYWORD: unnamed_7 = 5;
pub const MAILIMAP_FLAG_DRAFT: unnamed_7 = 4;
pub const MAILIMAP_FLAG_SEEN: unnamed_7 = 3;
pub const MAILIMAP_FLAG_DELETED: unnamed_7 = 2;
pub const MAILIMAP_FLAG_FLAGGED: unnamed_7 = 1;
pub const MAILIMAP_FLAG_ANSWERED: unnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_flag {
    pub fl_type: libc::c_int,
    pub fl_data: unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_8 {
    pub fl_keyword: *mut libc::c_char,
    pub fl_extension: *mut libc::c_char,
}
pub type unnamed_9 = libc::c_uint;
pub const MAILIMAP_FLAG_FETCH_OTHER: unnamed_9 = 2;
pub const MAILIMAP_FLAG_FETCH_RECENT: unnamed_9 = 1;
pub const MAILIMAP_FLAG_FETCH_ERROR: unnamed_9 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_flag_fetch {
    pub fl_type: libc::c_int,
    pub fl_flag: *mut mailimap_flag,
}
pub type unnamed_10 = libc::c_uint;
pub const MAILIMAP_FLAG_PERM_ALL: unnamed_10 = 2;
pub const MAILIMAP_FLAG_PERM_FLAG: unnamed_10 = 1;
pub const MAILIMAP_FLAG_PERM_ERROR: unnamed_10 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_flag_perm {
    pub fl_type: libc::c_int,
    pub fl_flag: *mut mailimap_flag,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_flag_list {
    pub fl_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_mailbox_data_status {
    pub st_mailbox: *mut libc::c_char,
    pub st_info_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_msg_att {
    pub att_list: *mut clist,
    pub att_number: uint32_t,
}
pub type unnamed_11 = libc::c_uint;
pub const MAILIMAP_MSG_ATT_ITEM_EXTENSION: unnamed_11 = 3;
pub const MAILIMAP_MSG_ATT_ITEM_STATIC: unnamed_11 = 2;
pub const MAILIMAP_MSG_ATT_ITEM_DYNAMIC: unnamed_11 = 1;
pub const MAILIMAP_MSG_ATT_ITEM_ERROR: unnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_msg_att_item {
    pub att_type: libc::c_int,
    pub att_data: unnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_12 {
    pub att_dyn: *mut mailimap_msg_att_dynamic,
    pub att_static: *mut mailimap_msg_att_static,
    pub att_extension_data: *mut mailimap_extension_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_msg_att_static {
    pub att_type: libc::c_int,
    pub att_data: unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_13 {
    pub att_env: *mut mailimap_envelope,
    pub att_internal_date: *mut mailimap_date_time,
    pub att_rfc822: unnamed_16,
    pub att_rfc822_header: unnamed_15,
    pub att_rfc822_text: unnamed_14,
    pub att_rfc822_size: uint32_t,
    pub att_bodystructure: *mut mailimap_body,
    pub att_body: *mut mailimap_body,
    pub att_body_section: *mut mailimap_msg_att_body_section,
    pub att_uid: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_14 {
    pub att_content: *mut libc::c_char,
    pub att_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_15 {
    pub att_content: *mut libc::c_char,
    pub att_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_16 {
    pub att_content: *mut libc::c_char,
    pub att_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_msg_att_dynamic {
    pub att_list: *mut clist,
}
pub type unnamed_17 = libc::c_uint;
pub const MAILIMAP_MSG_ATT_UID: unnamed_17 = 10;
pub const MAILIMAP_MSG_ATT_BODY_SECTION: unnamed_17 = 9;
pub const MAILIMAP_MSG_ATT_BODYSTRUCTURE: unnamed_17 = 8;
pub const MAILIMAP_MSG_ATT_BODY: unnamed_17 = 7;
pub const MAILIMAP_MSG_ATT_RFC822_SIZE: unnamed_17 = 6;
pub const MAILIMAP_MSG_ATT_RFC822_TEXT: unnamed_17 = 5;
pub const MAILIMAP_MSG_ATT_RFC822_HEADER: unnamed_17 = 4;
pub const MAILIMAP_MSG_ATT_RFC822: unnamed_17 = 3;
pub const MAILIMAP_MSG_ATT_INTERNALDATE: unnamed_17 = 2;
pub const MAILIMAP_MSG_ATT_ENVELOPE: unnamed_17 = 1;
pub const MAILIMAP_MSG_ATT_ERROR: unnamed_17 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_set_item {
    pub set_first: uint32_t,
    pub set_last: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_set {
    pub set_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_fetch_att {
    pub att_type: libc::c_int,
    pub att_section: *mut mailimap_section,
    pub att_offset: uint32_t,
    pub att_size: uint32_t,
    pub att_extension: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_fetch_type {
    pub ft_type: libc::c_int,
    pub ft_data: unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_18 {
    pub ft_fetch_att: *mut mailimap_fetch_att,
    pub ft_fetch_att_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_store_att_flags {
    pub fl_sign: libc::c_int,
    pub fl_silent: libc::c_int,
    pub fl_flag_list: *mut mailimap_flag_list,
}
pub type mailimap_msg_att_handler =
    unsafe extern "C" fn(_: *mut mailimap_msg_att, _: *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap {
    pub imap_response: *mut libc::c_char,
    pub imap_stream: *mut mailstream,
    pub imap_progr_rate: size_t,
    pub imap_progr_fun: Option<unsafe extern "C" fn(_: size_t, _: size_t) -> ()>,
    pub imap_stream_buffer: *mut MMAPString,
    pub imap_response_buffer: *mut MMAPString,
    pub imap_state: libc::c_int,
    pub imap_tag: libc::c_int,
    pub imap_connection_info: *mut mailimap_connection_info,
    pub imap_selection_info: *mut mailimap_selection_info,
    pub imap_response_info: *mut mailimap_response_info,
    pub imap_sasl: unnamed_19,
    pub imap_idle_timestamp: time_t,
    pub imap_idle_maxdelay: time_t,
    pub imap_body_progress_fun:
        Option<unsafe extern "C" fn(_: size_t, _: size_t, _: *mut libc::c_void) -> ()>,
    pub imap_items_progress_fun:
        Option<unsafe extern "C" fn(_: size_t, _: size_t, _: *mut libc::c_void) -> ()>,
    pub imap_progress_context: *mut libc::c_void,
    pub imap_msg_att_handler:
        Option<unsafe extern "C" fn(_: *mut mailimap_msg_att, _: *mut libc::c_void) -> ()>,
    pub imap_msg_att_handler_context: *mut libc::c_void,
    pub imap_msg_body_handler: Option<
        unsafe extern "C" fn(
            _: libc::c_int,
            _: *mut mailimap_msg_att_body_section,
            _: *const libc::c_char,
            _: size_t,
            _: *mut libc::c_void,
        ) -> bool,
    >,
    pub imap_msg_body_handler_context: *mut libc::c_void,
    pub imap_timeout: time_t,
    pub imap_logger: Option<
        unsafe extern "C" fn(
            _: *mut mailimap,
            _: libc::c_int,
            _: *const libc::c_char,
            _: size_t,
            _: *mut libc::c_void,
        ) -> (),
    >,
    pub imap_logger_context: *mut libc::c_void,
    pub is_163_workaround_enabled: libc::c_int,
    pub is_rambler_workaround_enabled: libc::c_int,
    pub is_qip_workaround_enabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_19 {
    pub sasl_conn: *mut libc::c_void,
    pub sasl_server_fqdn: *const libc::c_char,
    pub sasl_login: *const libc::c_char,
    pub sasl_auth_name: *const libc::c_char,
    pub sasl_password: *const libc::c_char,
    pub sasl_realm: *const libc::c_char,
    pub sasl_secret: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_response_info {
    pub rsp_alert: *mut libc::c_char,
    pub rsp_parse: *mut libc::c_char,
    pub rsp_badcharset: *mut clist,
    pub rsp_trycreate: libc::c_int,
    pub rsp_mailbox_list: *mut clist,
    pub rsp_mailbox_lsub: *mut clist,
    pub rsp_search_result: *mut clist,
    pub rsp_status: *mut mailimap_mailbox_data_status,
    pub rsp_expunged: *mut clist,
    pub rsp_fetch_list: *mut clist,
    pub rsp_extension_list: *mut clist,
    pub rsp_atom: *mut libc::c_char,
    pub rsp_value: *mut libc::c_char,
}
#[derive(BitfieldStruct, Clone, Copy)]
#[repr(C)]
pub struct mailimap_selection_info {
    pub sel_perm_flags: *mut clist,
    pub sel_perm: libc::c_int,
    pub sel_uidnext: uint32_t,
    pub sel_uidvalidity: uint32_t,
    pub sel_first_unseen: uint32_t,
    pub sel_flags: *mut mailimap_flag_list,
    pub sel_exists: uint32_t,
    pub sel_recent: uint32_t,
    pub sel_unseen: uint32_t,
    #[bitfield(name = "sel_has_exists", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "sel_has_recent", ty = "uint8_t", bits = "1..=1")]
    pub sel_has_exists_sel_has_recent: [u8; 1],
    pub _pad: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimap_connection_info {
    pub imap_capability: *mut mailimap_capability_data,
}
pub type unnamed_20 = libc::c_uint;
pub const MAILIMAP_ERROR_CLIENTID: unnamed_20 = 46;
pub const MAILIMAP_ERROR_CUSTOM_COMMAND: unnamed_20 = 45;
pub const MAILIMAP_ERROR_NEEDS_MORE_DATA: unnamed_20 = 44;
pub const MAILIMAP_ERROR_SSL: unnamed_20 = 43;
pub const MAILIMAP_ERROR_SASL: unnamed_20 = 42;
pub const MAILIMAP_ERROR_EXTENSION: unnamed_20 = 41;
pub const MAILIMAP_ERROR_INVAL: unnamed_20 = 40;
pub const MAILIMAP_ERROR_STARTTLS: unnamed_20 = 39;
pub const MAILIMAP_ERROR_UNSUBSCRIBE: unnamed_20 = 38;
pub const MAILIMAP_ERROR_SUBSCRIBE: unnamed_20 = 37;
pub const MAILIMAP_ERROR_UID_STORE: unnamed_20 = 36;
pub const MAILIMAP_ERROR_STORE: unnamed_20 = 35;
pub const MAILIMAP_ERROR_STATUS: unnamed_20 = 34;
pub const MAILIMAP_ERROR_SELECT: unnamed_20 = 33;
pub const MAILIMAP_ERROR_UID_SEARCH: unnamed_20 = 32;
pub const MAILIMAP_ERROR_SEARCH: unnamed_20 = 31;
pub const MAILIMAP_ERROR_RENAME: unnamed_20 = 30;
pub const MAILIMAP_ERROR_LSUB: unnamed_20 = 29;
pub const MAILIMAP_ERROR_LOGIN: unnamed_20 = 28;
pub const MAILIMAP_ERROR_LIST: unnamed_20 = 27;
pub const MAILIMAP_ERROR_UID_FETCH: unnamed_20 = 26;
pub const MAILIMAP_ERROR_FETCH: unnamed_20 = 25;
pub const MAILIMAP_ERROR_EXAMINE: unnamed_20 = 24;
pub const MAILIMAP_ERROR_DELETE: unnamed_20 = 23;
pub const MAILIMAP_ERROR_CREATE: unnamed_20 = 22;
pub const MAILIMAP_ERROR_UID_MOVE: unnamed_20 = 21;
pub const MAILIMAP_ERROR_MOVE: unnamed_20 = 20;
pub const MAILIMAP_ERROR_UID_COPY: unnamed_20 = 19;
pub const MAILIMAP_ERROR_COPY: unnamed_20 = 18;
pub const MAILIMAP_ERROR_EXPUNGE: unnamed_20 = 17;
pub const MAILIMAP_ERROR_CLOSE: unnamed_20 = 16;
pub const MAILIMAP_ERROR_CHECK: unnamed_20 = 15;
pub const MAILIMAP_ERROR_CAPABILITY: unnamed_20 = 14;
pub const MAILIMAP_ERROR_LOGOUT: unnamed_20 = 13;
pub const MAILIMAP_ERROR_NOOP: unnamed_20 = 12;
pub const MAILIMAP_ERROR_APPEND: unnamed_20 = 11;
pub const MAILIMAP_ERROR_DONT_ACCEPT_CONNECTION: unnamed_20 = 10;
pub const MAILIMAP_ERROR_PROTOCOL: unnamed_20 = 9;
pub const MAILIMAP_ERROR_FATAL: unnamed_20 = 8;
pub const MAILIMAP_ERROR_MEMORY: unnamed_20 = 7;
pub const MAILIMAP_ERROR_CONNECTION_REFUSED: unnamed_20 = 6;
pub const MAILIMAP_ERROR_PARSE: unnamed_20 = 5;
pub const MAILIMAP_ERROR_STREAM: unnamed_20 = 4;
pub const MAILIMAP_ERROR_BAD_STATE: unnamed_20 = 3;
pub const MAILIMAP_NO_ERROR_NON_AUTHENTICATED: unnamed_20 = 2;
pub const MAILIMAP_NO_ERROR_AUTHENTICATED: unnamed_20 = 1;
pub const MAILIMAP_NO_ERROR: unnamed_20 = 0;
/* define DC_USE_RPGP to enable use of rPGP instead of netpgp where available;
preferrably, this should be done in the project configuration currently */
//#define DC_USE_RPGP 1
/* Includes that are used frequently.  This file may also be used to create predefined headers. */
/* * Structure behind dc_context_t */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_context {
    pub magic: uint32_t,
    pub userdata: *mut libc::c_void,
    pub dbfile: *mut libc::c_char,
    pub blobdir: *mut libc::c_char,
    pub sql: *mut dc_sqlite3_t,
    pub inbox: *mut dc_imap_t,
    pub inboxidle_condmutex: pthread_mutex_t,
    pub perform_inbox_jobs_needed: libc::c_int,
    pub probe_imap_network: libc::c_int,
    pub sentbox_thread: dc_jobthread_t,
    pub mvbox_thread: dc_jobthread_t,
    pub smtp: *mut dc_smtp_t,
    pub smtpidle_cond: pthread_cond_t,
    pub smtpidle_condmutex: pthread_mutex_t,
    pub smtpidle_condflag: libc::c_int,
    pub smtp_suspended: libc::c_int,
    pub smtp_doing_jobs: libc::c_int,
    pub perform_smtp_jobs_needed: libc::c_int,
    pub probe_smtp_network: libc::c_int,
    pub oauth2_critical: pthread_mutex_t,
    pub cb: dc_callback_t,
    pub os_name: *mut libc::c_char,
    pub cmdline_sel_chat_id: uint32_t,
    pub bob_expects: libc::c_int,
    pub bobs_status: libc::c_int,
    pub bobs_qr_scan: *mut dc_lot_t,
    pub bobs_qr_critical: pthread_mutex_t,
    pub last_smeared_timestamp: time_t,
    pub smear_critical: pthread_mutex_t,
    pub ongoing_running: libc::c_int,
    pub shall_stop_ongoing: libc::c_int,
}
pub type dc_lot_t = _dc_lot;
/* * Structure behind dc_lot_t */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_lot {
    pub magic: uint32_t,
    pub text1_meaning: libc::c_int,
    pub text1: *mut libc::c_char,
    pub text2: *mut libc::c_char,
    pub timestamp: time_t,
    pub state: libc::c_int,
    pub id: uint32_t,
    pub fingerprint: *mut libc::c_char,
    pub invitenumber: *mut libc::c_char,
    pub auth: *mut libc::c_char,
}
/* *
 * Callback function that should be given to dc_context_new().
 *
 * @memberof dc_context_t
 * @param context The context object as returned by dc_context_new().
 * @param event one of the @ref DC_EVENT constants
 * @param data1 depends on the event parameter
 * @param data2 depends on the event parameter
 * @return return 0 unless stated otherwise in the event parameter documentation
 */
pub type dc_callback_t = Option<
    unsafe extern "C" fn(
        _: *mut dc_context_t,
        _: libc::c_int,
        _: uintptr_t,
        _: uintptr_t,
    ) -> uintptr_t,
>;
/* *
 * @mainpage Getting started
 *
 * This document describes how to handle the Delta Chat core library.
 * For general information about Delta Chat itself,
 * see <https://delta.chat> and <https://github.com/deltachat>.
 *
 * Let's start.
 *
 * First of all, you have to **define an event-handler-function**
 * that is called by the library on specific events
 * (eg. when the configuration is done or when fresh messages arrive).
 * With this function you can create a Delta Chat context then:
 *
 * ~~~
 * #include <deltachat.h>
 *
 * uintptr_t event_handler_func(dc_context_t* context, int event,
 *                              uintptr_t data1, uintptr_t data2)
 * {
 *     return 0; // for unhandled events, it is always safe to return 0
 * }
 *
 * dc_context_t* context = dc_context_new(event_handler_func, NULL, NULL);
 * ~~~
 *
 * After that, you should make sure,
 * sending and receiving jobs are processed as needed.
 * For this purpose, you have to **create two threads:**
 *
 * ~~~
 * #include <pthread.h>
 *
 * void* imap_thread_func(void* context)
 * {
 *     while (true) {
 *         dc_perform_imap_jobs(context);
 *         dc_perform_imap_fetch(context);
 *         dc_perform_imap_idle(context);
 *     }
 * }
 *
 * void* smtp_thread_func(void* context)
 * {
 *     while (true) {
 *         dc_perform_smtp_jobs(context);
 *         dc_perform_smtp_idle(context);
 *     }
 * }
 *
 * static pthread_t imap_thread, smtp_thread;
 * pthread_create(&imap_thread, NULL, imap_thread_func, context);
 * pthread_create(&smtp_thread, NULL, smtp_thread_func, context);
 * ~~~
 *
 * The example above uses "pthreads",
 * however, you can also use anything else for thread handling.
 * NB: The deltachat-core library itself does not create any threads on its own,
 * however, functions, unless stated otherwise, are thread-safe.
 *
 * After that you can  **define and open a database.**
 * The database is a normal sqlite-file and is created as needed:
 *
 * ~~~
 * dc_open(context, "example.db", NULL);
 * ~~~
 *
 * Now you can **configure the context:**
 *
 * ~~~
 * // use some real test credentials here
 * dc_set_config(context, "addr", "alice@example.org");
 * dc_set_config(context, "mail_pw", "***");
 * dc_configure(context);
 * ~~~
 *
 * dc_configure() returns immediately, the configuration itself may take a while
 * and is done by a job in the imap-thread you've defined above.
 * Once done, the #DC_EVENT_CONFIGURE_PROGRESS reports success
 * to the event_handler_func() that is also defined above.
 *
 * The configuration result is saved in the database,
 * on subsequent starts it is not needed to call dc_configure()
 * (you can check this using dc_is_configured()).
 *
 * Now you can **send the first message:**
 *
 * ~~~
 * // use a real testing address here
 * uint32_t contact_id = dc_create_contact(context, NULL, "bob@example.org");
 * uint32_t chat_id    = dc_create_chat_by_contact_id(context, contact_id);
 *
 * dc_send_text_msg(context, chat_id, "Hi, here is my first message!");
 * ~~~
 *
 * dc_send_text_msg() returns immediately;
 * the sending itself is done by a job in the smtp-thread you've defined above.
 * If you check the testing address (bob)
 * and you should have received a normal email.
 * Answer this email in any email program with "Got it!"
 * and the imap-thread you've create above will **receive the message**.
 *
 * You can then **list all messages** of a chat as follow:
 *
 * ~~~
 * dc_array_t* msglist = dc_get_chat_msgs(context, chat_id, 0, 0);
 * for (int i = 0; i < dc_array_get_cnt(msglist); i++)
 * {
 *     uint32_t  msg_id = dc_array_get_id(msglist, i);
 *     dc_msg_t* msg    = dc_get_msg(context, msg_id);
 *     char*     text   = dc_msg_get_text(msg);
 *
 *     printf("Message %i: %s\n", i+1, text);
 *
 *     free(text);
 *     dc_msg_unref(msg);
 * }
 * dc_array_unref(msglist);
 * ~~~
 *
 * This will output the following two lines:
 *
 * ~~~
 * Message 1: Hi, here is my first message!
 * Message 2: Got it!
 * ~~~
 *
 *
 * ## Class reference
 *
 * For a class reference, see the "Classes" link atop.
 *
 *
 * ## Further hints
 *
 * Here are some additional, unsorted hints that may be useful.
 *
 * - For `get`-functions, you have to unref the return value in some way.
 *
 * - Strings in function arguments or return values are usually UTF-8 encoded.
 *
 * - The issue-tracker for the core library is here:
 *   <https://github.com/deltachat/deltachat-core/issues>
 *
 * The following points are important mainly
 * for the authors of the library itself:
 *
 * - For indentation, use tabs.
 *   Alignments that are not placed at the beginning of a line
 *   should be done with spaces.
 *
 * - For padding between functions,
 *   classes etc. use 2 empty lines
 *
 * - Source files are encoded as UTF-8 with Unix line endings
 *   (a simple `LF`, `0x0A` or `\n`)
 *
 * If you need further assistance,
 * please do not hesitate to contact us
 * through the channels shown at https://delta.chat/en/contribute
 *
 * Please keep in mind, that your derived work
 * must respect the Mozilla Public License 2.0 of libdeltachat
 * and the respective licenses of the libraries libdeltachat links with.
 *
 * See you.
 */
/* *
 * @class dc_context_t
 *
 * An object representing a single account.
 *
 * Each account is linked to an IMAP/SMTP account and uses a separate
 * SQLite database for offline functionality and for account-related
 * settings.
 */
pub type dc_context_t = _dc_context;
/* ** library-private **********************************************************/
pub type dc_smtp_t = _dc_smtp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_smtp {
    pub etpan: *mut mailsmtp,
    pub from: *mut libc::c_char,
    pub esmtp: libc::c_int,
    pub log_connect_errors: libc::c_int,
    pub context: *mut dc_context_t,
    pub error: *mut libc::c_char,
    pub error_etpan: libc::c_int,
}
pub type dc_jobthread_t = _dc_jobthread;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_jobthread {
    pub context: *mut dc_context_t,
    pub name: *mut libc::c_char,
    pub folder_config_name: *mut libc::c_char,
    pub imap: *mut _dc_imap,
    pub mutex: pthread_mutex_t,
    pub idle_cond: pthread_cond_t,
    pub idle_condflag: libc::c_int,
    pub jobs_needed: libc::c_int,
    pub suspended: libc::c_int,
    pub using_handle: libc::c_int,
}
/* *
 * Library-internal.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_imap {
    pub addr: *mut libc::c_char,
    pub imap_server: *mut libc::c_char,
    pub imap_port: libc::c_int,
    pub imap_user: *mut libc::c_char,
    pub imap_pw: *mut libc::c_char,
    pub server_flags: libc::c_int,
    pub connected: libc::c_int,
    pub etpan: *mut mailimap,
    pub idle_set_up: libc::c_int,
    pub selected_folder: *mut libc::c_char,
    pub selected_folder_needs_expunge: libc::c_int,
    pub should_reconnect: libc::c_int,
    pub can_idle: libc::c_int,
    pub has_xlist: libc::c_int,
    pub imap_delimiter: libc::c_char,
    pub watch_folder: *mut libc::c_char,
    pub watch_cond: pthread_cond_t,
    pub watch_condmutex: pthread_mutex_t,
    pub watch_condflag: libc::c_int,
    pub fetch_type_prefetch: *mut mailimap_fetch_type,
    pub fetch_type_body: *mut mailimap_fetch_type,
    pub fetch_type_flags: *mut mailimap_fetch_type,
    pub get_config: dc_get_config_t,
    pub set_config: dc_set_config_t,
    pub precheck_imf: dc_precheck_imf_t,
    pub receive_imf: dc_receive_imf_t,
    pub userData: *mut libc::c_void,
    pub context: *mut dc_context_t,
    pub log_connect_errors: libc::c_int,
    pub skip_log_capabilities: libc::c_int,
}
pub type dc_receive_imf_t = Option<
    unsafe extern "C" fn(
        _: *mut dc_imap_t,
        _: *const libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: uint32_t,
        _: uint32_t,
    ) -> (),
>;
/* Purpose: Reading from IMAP servers with no dependencies to the database.
dc_context_t is only used for logging and to get information about
the online state. */
pub type dc_imap_t = _dc_imap;
pub type dc_precheck_imf_t = Option<
    unsafe extern "C" fn(
        _: *mut dc_imap_t,
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: uint32_t,
    ) -> libc::c_int,
>;
pub type dc_set_config_t = Option<
    unsafe extern "C" fn(_: *mut dc_imap_t, _: *const libc::c_char, _: *const libc::c_char) -> (),
>;
pub type dc_get_config_t = Option<
    unsafe extern "C" fn(
        _: *mut dc_imap_t,
        _: *const libc::c_char,
        _: *const libc::c_char,
    ) -> *mut libc::c_char,
>;
/* ** library-private **********************************************************/
pub type dc_sqlite3_t = _dc_sqlite3;
/* *
 * Library-internal.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_sqlite3 {
    pub cobj: *mut sqlite3,
    pub context: *mut dc_context_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_strbuilder {
    pub buf: *mut libc::c_char,
    pub allocated: libc::c_int,
    pub free: libc::c_int,
    pub eos: *mut libc::c_char,
}
pub type dc_strbuilder_t = _dc_strbuilder;
/* *
 * Library-internal.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dc_loginparam {
    pub addr: *mut libc::c_char,
    pub mail_server: *mut libc::c_char,
    pub mail_user: *mut libc::c_char,
    pub mail_pw: *mut libc::c_char,
    pub mail_port: uint16_t,
    pub send_server: *mut libc::c_char,
    pub send_user: *mut libc::c_char,
    pub send_pw: *mut libc::c_char,
    pub send_port: libc::c_int,
    pub server_flags: libc::c_int,
}
pub type dc_loginparam_t = _dc_loginparam;
pub type dc_imap_res = libc::c_uint;
pub const DC_SUCCESS: dc_imap_res = 3;
pub const DC_ALREADY_DONE: dc_imap_res = 2;
pub const DC_RETRY_LATER: dc_imap_res = 1;
pub const DC_FAILED: dc_imap_res = 0;
#[no_mangle]
pub unsafe extern "C" fn dc_imap_new(
    mut get_config: dc_get_config_t,
    mut set_config: dc_set_config_t,
    mut precheck_imf: dc_precheck_imf_t,
    mut receive_imf: dc_receive_imf_t,
    mut userData: *mut libc::c_void,
    mut context: *mut dc_context_t,
) -> *mut dc_imap_t {
    let mut imap: *mut dc_imap_t = 0 as *mut dc_imap_t;
    imap = calloc(
        1i32 as libc::c_ulong,
        ::std::mem::size_of::<dc_imap_t>() as libc::c_ulong,
    ) as *mut dc_imap_t;
    if imap.is_null() {
        exit(25i32);
    }
    (*imap).log_connect_errors = 1i32;
    (*imap).context = context;
    (*imap).get_config = get_config;
    (*imap).set_config = set_config;
    (*imap).precheck_imf = precheck_imf;
    (*imap).receive_imf = receive_imf;
    (*imap).userData = userData;
    pthread_mutex_init(
        &mut (*imap).watch_condmutex,
        0 as *const pthread_mutexattr_t,
    );
    pthread_cond_init(&mut (*imap).watch_cond, 0 as *const pthread_condattr_t);
    (*imap).watch_folder =
        calloc(1i32 as libc::c_ulong, 1i32 as libc::c_ulong) as *mut libc::c_char;
    (*imap).selected_folder =
        calloc(1i32 as libc::c_ulong, 1i32 as libc::c_ulong) as *mut libc::c_char;
    (*imap).fetch_type_prefetch = mailimap_fetch_type_new_fetch_att_list_empty();
    mailimap_fetch_type_new_fetch_att_list_add(
        (*imap).fetch_type_prefetch,
        mailimap_fetch_att_new_uid(),
    );
    mailimap_fetch_type_new_fetch_att_list_add(
        (*imap).fetch_type_prefetch,
        mailimap_fetch_att_new_envelope(),
    );
    (*imap).fetch_type_body = mailimap_fetch_type_new_fetch_att_list_empty();
    mailimap_fetch_type_new_fetch_att_list_add(
        (*imap).fetch_type_body,
        mailimap_fetch_att_new_flags(),
    );
    mailimap_fetch_type_new_fetch_att_list_add(
        (*imap).fetch_type_body,
        mailimap_fetch_att_new_body_peek_section(mailimap_section_new(
            0 as *mut mailimap_section_spec,
        )),
    );
    (*imap).fetch_type_flags = mailimap_fetch_type_new_fetch_att_list_empty();
    mailimap_fetch_type_new_fetch_att_list_add(
        (*imap).fetch_type_flags,
        mailimap_fetch_att_new_flags(),
    );
    return imap;
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_unref(mut imap: *mut dc_imap_t) {
    if imap.is_null() {
        return;
    }
    dc_imap_disconnect(imap);
    pthread_cond_destroy(&mut (*imap).watch_cond);
    pthread_mutex_destroy(&mut (*imap).watch_condmutex);
    free((*imap).watch_folder as *mut libc::c_void);
    free((*imap).selected_folder as *mut libc::c_void);
    if !(*imap).fetch_type_prefetch.is_null() {
        mailimap_fetch_type_free((*imap).fetch_type_prefetch);
    }
    if !(*imap).fetch_type_body.is_null() {
        mailimap_fetch_type_free((*imap).fetch_type_body);
    }
    if !(*imap).fetch_type_flags.is_null() {
        mailimap_fetch_type_free((*imap).fetch_type_flags);
    }
    free(imap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_disconnect(mut imap: *mut dc_imap_t) {
    if imap.is_null() {
        return;
    }
    if 0 != (*imap).connected {
        unsetup_handle(imap);
        free_connect_param(imap);
        (*imap).connected = 0i32
    };
}
/* we leave sent_folder set; normally this does not change in a normal reconnect; we'll update this folder if we get errors */
/* ******************************************************************************
 * Connect/Disconnect
 ******************************************************************************/
unsafe extern "C" fn free_connect_param(mut imap: *mut dc_imap_t) {
    free((*imap).addr as *mut libc::c_void);
    (*imap).addr = 0 as *mut libc::c_char;
    free((*imap).imap_server as *mut libc::c_void);
    (*imap).imap_server = 0 as *mut libc::c_char;
    free((*imap).imap_user as *mut libc::c_void);
    (*imap).imap_user = 0 as *mut libc::c_char;
    free((*imap).imap_pw as *mut libc::c_void);
    (*imap).imap_pw = 0 as *mut libc::c_char;
    *(*imap).watch_folder.offset(0isize) = 0i32 as libc::c_char;
    *(*imap).selected_folder.offset(0isize) = 0i32 as libc::c_char;
    (*imap).imap_port = 0i32;
    (*imap).can_idle = 0i32;
    (*imap).has_xlist = 0i32;
}
unsafe extern "C" fn unsetup_handle(mut imap: *mut dc_imap_t) {
    if imap.is_null() {
        return;
    }
    if !(*imap).etpan.is_null() {
        if 0 != (*imap).idle_set_up {
            mailstream_unsetup_idle((*(*imap).etpan).imap_stream);
            (*imap).idle_set_up = 0i32
        }
        if !(*(*imap).etpan).imap_stream.is_null() {
            mailstream_close((*(*imap).etpan).imap_stream);
            (*(*imap).etpan).imap_stream = 0 as *mut mailstream
        }
        mailimap_free((*imap).etpan);
        (*imap).etpan = 0 as *mut mailimap;
        dc_log_info(
            (*imap).context,
            0i32,
            b"IMAP disconnected.\x00" as *const u8 as *const libc::c_char,
        );
    }
    *(*imap).selected_folder.offset(0isize) = 0i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_connect(
    mut imap: *mut dc_imap_t,
    mut lp: *const dc_loginparam_t,
) -> libc::c_int {
    let mut success: libc::c_int = 0i32;
    if imap.is_null()
        || lp.is_null()
        || (*lp).mail_server.is_null()
        || (*lp).mail_user.is_null()
        || (*lp).mail_pw.is_null()
    {
        return 0i32;
    }
    if 0 != (*imap).connected {
        success = 1i32
    } else {
        (*imap).addr = dc_strdup((*lp).addr);
        (*imap).imap_server = dc_strdup((*lp).mail_server);
        (*imap).imap_port = (*lp).mail_port as libc::c_int;
        (*imap).imap_user = dc_strdup((*lp).mail_user);
        (*imap).imap_pw = dc_strdup((*lp).mail_pw);
        (*imap).server_flags = (*lp).server_flags;
        if !(0 == setup_handle_if_needed(imap)) {
            (*imap).can_idle = mailimap_has_idle((*imap).etpan);
            (*imap).has_xlist = mailimap_has_xlist((*imap).etpan);
            (*imap).can_idle = 0i32;
            if 0 == (*imap).skip_log_capabilities
                && !(*(*imap).etpan).imap_connection_info.is_null()
                && !(*(*(*imap).etpan).imap_connection_info)
                    .imap_capability
                    .is_null()
            {
                (*imap).skip_log_capabilities = 1i32;
                let mut capinfostr: dc_strbuilder_t = _dc_strbuilder {
                    buf: 0 as *mut libc::c_char,
                    allocated: 0,
                    free: 0,
                    eos: 0 as *mut libc::c_char,
                };
                dc_strbuilder_init(&mut capinfostr, 0i32);
                let mut list: *mut clist =
                    (*(*(*(*imap).etpan).imap_connection_info).imap_capability).cap_list;
                if !list.is_null() {
                    let mut cur: *mut clistiter = 0 as *mut clistiter;
                    cur = (*list).first;
                    while !cur.is_null() {
                        let mut cap: *mut mailimap_capability = (if !cur.is_null() {
                            (*cur).data
                        } else {
                            0 as *mut libc::c_void
                        })
                            as *mut mailimap_capability;
                        if !cap.is_null()
                            && (*cap).cap_type == MAILIMAP_CAPABILITY_NAME as libc::c_int
                        {
                            dc_strbuilder_cat(
                                &mut capinfostr,
                                b" \x00" as *const u8 as *const libc::c_char,
                            );
                            dc_strbuilder_cat(&mut capinfostr, (*cap).cap_data.cap_name);
                        }
                        cur = if !cur.is_null() {
                            (*cur).next
                        } else {
                            0 as *mut clistcell_s
                        }
                    }
                }
                dc_log_info(
                    (*imap).context,
                    0i32,
                    b"IMAP-capabilities:%s\x00" as *const u8 as *const libc::c_char,
                    capinfostr.buf,
                );
                free(capinfostr.buf as *mut libc::c_void);
            }
            (*imap).connected = 1i32;
            success = 1i32
        }
    }
    if success == 0i32 {
        unsetup_handle(imap);
        free_connect_param(imap);
    }
    return success;
}
unsafe extern "C" fn setup_handle_if_needed(mut imap: *mut dc_imap_t) -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0i32;
    let mut success: libc::c_int = 0i32;
    if !(imap.is_null() || (*imap).imap_server.is_null()) {
        if 0 != (*imap).should_reconnect {
            unsetup_handle(imap);
        }
        if !(*imap).etpan.is_null() {
            success = 1i32
        } else {
            (*imap).etpan = mailimap_new(0i32 as size_t, None);
            mailimap_set_timeout((*imap).etpan, 10i32 as time_t);
            if 0 != (*imap).server_flags & (0x100i32 | 0x400i32) {
                r = mailimap_socket_connect(
                    (*imap).etpan,
                    (*imap).imap_server,
                    (*imap).imap_port as uint16_t,
                );
                if 0 != dc_imap_is_error(imap, r) {
                    dc_log_event_seq(
                        (*imap).context,
                        401i32,
                        &mut (*imap).log_connect_errors as *mut libc::c_int,
                        b"Could not connect to IMAP-server %s:%i. (Error #%i)\x00" as *const u8
                            as *const libc::c_char,
                        (*imap).imap_server,
                        (*imap).imap_port as libc::c_int,
                        r as libc::c_int,
                    );
                    current_block = 15811161807000851472;
                } else if 0 != (*imap).server_flags & 0x100i32 {
                    r = mailimap_socket_starttls((*imap).etpan);
                    if 0 != dc_imap_is_error(imap, r) {
                        dc_log_event_seq((*imap).context, 401i32,
                                         &mut (*imap).log_connect_errors as
                                             *mut libc::c_int,
                                         b"Could not connect to IMAP-server %s:%i using STARTTLS. (Error #%i)\x00"
                                             as *const u8 as
                                             *const libc::c_char,
                                         (*imap).imap_server,
                                         (*imap).imap_port as libc::c_int,
                                         r as libc::c_int);
                        current_block = 15811161807000851472;
                    } else {
                        dc_log_info(
                            (*imap).context,
                            0i32,
                            b"IMAP-server %s:%i STARTTLS-connected.\x00" as *const u8
                                as *const libc::c_char,
                            (*imap).imap_server,
                            (*imap).imap_port as libc::c_int,
                        );
                        current_block = 14763689060501151050;
                    }
                } else {
                    dc_log_info(
                        (*imap).context,
                        0i32,
                        b"IMAP-server %s:%i connected.\x00" as *const u8 as *const libc::c_char,
                        (*imap).imap_server,
                        (*imap).imap_port as libc::c_int,
                    );
                    current_block = 14763689060501151050;
                }
            } else {
                r = mailimap_ssl_connect(
                    (*imap).etpan,
                    (*imap).imap_server,
                    (*imap).imap_port as uint16_t,
                );
                if 0 != dc_imap_is_error(imap, r) {
                    dc_log_event_seq(
                        (*imap).context,
                        401i32,
                        &mut (*imap).log_connect_errors as *mut libc::c_int,
                        b"Could not connect to IMAP-server %s:%i using SSL. (Error #%i)\x00"
                            as *const u8 as *const libc::c_char,
                        (*imap).imap_server,
                        (*imap).imap_port as libc::c_int,
                        r as libc::c_int,
                    );
                    current_block = 15811161807000851472;
                } else {
                    dc_log_info(
                        (*imap).context,
                        0i32,
                        b"IMAP-server %s:%i SSL-connected.\x00" as *const u8 as *const libc::c_char,
                        (*imap).imap_server,
                        (*imap).imap_port as libc::c_int,
                    );
                    current_block = 14763689060501151050;
                }
            }
            match current_block {
                15811161807000851472 => {}
                _ => {
                    if 0 != (*imap).server_flags & 0x2i32 {
                        dc_log_info(
                            (*imap).context,
                            0i32,
                            b"IMAP-OAuth2 connect...\x00" as *const u8 as *const libc::c_char,
                        );
                        let mut access_token: *mut libc::c_char = dc_get_oauth2_access_token(
                            (*imap).context,
                            (*imap).addr,
                            (*imap).imap_pw,
                            0i32,
                        );
                        r = mailimap_oauth2_authenticate(
                            (*imap).etpan,
                            (*imap).imap_user,
                            access_token,
                        );
                        if 0 != dc_imap_is_error(imap, r) {
                            free(access_token as *mut libc::c_void);
                            access_token = dc_get_oauth2_access_token(
                                (*imap).context,
                                (*imap).addr,
                                (*imap).imap_pw,
                                0x1i32,
                            );
                            r = mailimap_oauth2_authenticate(
                                (*imap).etpan,
                                (*imap).imap_user,
                                access_token,
                            )
                        }
                        free(access_token as *mut libc::c_void);
                    } else {
                        r = mailimap_login((*imap).etpan, (*imap).imap_user, (*imap).imap_pw)
                    }
                    if 0 != dc_imap_is_error(imap, r) {
                        let mut msg: *mut libc::c_char = get_error_msg(
                            imap,
                            b"Cannot login\x00" as *const u8 as *const libc::c_char,
                            r,
                        );
                        dc_log_event_seq(
                            (*imap).context,
                            401i32,
                            &mut (*imap).log_connect_errors as *mut libc::c_int,
                            b"%s\x00" as *const u8 as *const libc::c_char,
                            msg,
                        );
                        free(msg as *mut libc::c_void);
                    } else {
                        dc_log_event(
                            (*imap).context,
                            102i32,
                            0i32,
                            b"IMAP-login as %s ok.\x00" as *const u8 as *const libc::c_char,
                            (*imap).imap_user,
                        );
                        success = 1i32
                    }
                }
            }
        }
    }
    if success == 0i32 {
        unsetup_handle(imap);
    }
    (*imap).should_reconnect = 0i32;
    return success;
}
unsafe extern "C" fn get_error_msg(
    mut imap: *mut dc_imap_t,
    mut what_failed: *const libc::c_char,
    mut code: libc::c_int,
) -> *mut libc::c_char {
    let mut stock: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg: dc_strbuilder_t = _dc_strbuilder {
        buf: 0 as *mut libc::c_char,
        allocated: 0,
        free: 0,
        eos: 0 as *mut libc::c_char,
    };
    dc_strbuilder_init(&mut msg, 1000i32);
    match code {
        28 => {
            stock = dc_stock_str_repl_string((*imap).context, 60i32, (*imap).imap_user);
            dc_strbuilder_cat(&mut msg, stock);
        }
        _ => {
            dc_strbuilder_catf(
                &mut msg as *mut dc_strbuilder_t,
                b"%s, IMAP-error #%i\x00" as *const u8 as *const libc::c_char,
                what_failed,
                code,
            );
        }
    }
    free(stock as *mut libc::c_void);
    stock = 0 as *mut libc::c_char;
    if !(*(*imap).etpan).imap_response.is_null() {
        dc_strbuilder_cat(&mut msg, b"\n\n\x00" as *const u8 as *const libc::c_char);
        stock = dc_stock_str_repl_string2(
            (*imap).context,
            61i32,
            (*imap).imap_server,
            (*(*imap).etpan).imap_response,
        );
        dc_strbuilder_cat(&mut msg, stock);
    }
    free(stock as *mut libc::c_void);
    stock = 0 as *mut libc::c_char;
    return msg.buf;
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_is_error(
    mut imap: *mut dc_imap_t,
    mut code: libc::c_int,
) -> libc::c_int {
    if code == MAILIMAP_NO_ERROR as libc::c_int
        || code == MAILIMAP_NO_ERROR_AUTHENTICATED as libc::c_int
        || code == MAILIMAP_NO_ERROR_NON_AUTHENTICATED as libc::c_int
    {
        return 0i32;
    }
    if code == MAILIMAP_ERROR_STREAM as libc::c_int || code == MAILIMAP_ERROR_PARSE as libc::c_int {
        dc_log_info(
            (*imap).context,
            0i32,
            b"IMAP stream lost; we\'ll reconnect soon.\x00" as *const u8 as *const libc::c_char,
        );
        (*imap).should_reconnect = 1i32
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_set_watch_folder(
    mut imap: *mut dc_imap_t,
    mut watch_folder: *const libc::c_char,
) {
    if imap.is_null() || watch_folder.is_null() {
        return;
    }
    free((*imap).watch_folder as *mut libc::c_void);
    (*imap).watch_folder = dc_strdup(watch_folder);
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_is_connected(mut imap: *const dc_imap_t) -> libc::c_int {
    return (!imap.is_null() && 0 != (*imap).connected) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_fetch(mut imap: *mut dc_imap_t) -> libc::c_int {
    let mut success: libc::c_int = 0i32;
    if !(imap.is_null() || 0 == (*imap).connected) {
        setup_handle_if_needed(imap);
        while fetch_from_single_folder(imap, (*imap).watch_folder) > 0i32 {}
        success = 1i32
    }
    return success;
}
unsafe extern "C" fn fetch_from_single_folder(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut uidvalidity: uint32_t = 0i32 as uint32_t;
    let mut lastseenuid: uint32_t = 0i32 as uint32_t;
    let mut new_lastseenuid: uint32_t = 0i32 as uint32_t;
    let mut fetch_result: *mut clist = 0 as *mut clist;
    let mut read_cnt: size_t = 0i32 as size_t;
    let mut read_errors: size_t = 0i32 as size_t;
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    let mut set: *mut mailimap_set = 0 as *mut mailimap_set;
    if !imap.is_null() {
        if (*imap).etpan.is_null() {
            dc_log_info(
                (*imap).context,
                0i32,
                b"Cannot fetch from \"%s\" - not connected.\x00" as *const u8
                    as *const libc::c_char,
                folder,
            );
        } else if select_folder(imap, folder) == 0i32 {
            dc_log_warning(
                (*imap).context,
                0i32,
                b"Cannot select folder %s for fetching.\x00" as *const u8 as *const libc::c_char,
                folder,
            );
        } else {
            get_config_lastseenuid(imap, folder, &mut uidvalidity, &mut lastseenuid);
            if uidvalidity != (*(*(*imap).etpan).imap_selection_info).sel_uidvalidity {
                /* first time this folder is selected or UIDVALIDITY has changed, init lastseenuid and save it to config */
                if (*(*(*imap).etpan).imap_selection_info).sel_uidvalidity <= 0i32 as libc::c_uint {
                    dc_log_error(
                        (*imap).context,
                        0i32,
                        b"Cannot get UIDVALIDITY for folder \"%s\".\x00" as *const u8
                            as *const libc::c_char,
                        folder,
                    );
                    current_block = 17288151659885296046;
                } else {
                    if 0 != (*(*(*imap).etpan).imap_selection_info).sel_has_exists() {
                        if (*(*(*imap).etpan).imap_selection_info).sel_exists
                            <= 0i32 as libc::c_uint
                        {
                            dc_log_info(
                                (*imap).context,
                                0i32,
                                b"Folder \"%s\" is empty.\x00" as *const u8 as *const libc::c_char,
                                folder,
                            );
                            if (*(*(*imap).etpan).imap_selection_info).sel_exists
                                == 0i32 as libc::c_uint
                            {
                                set_config_lastseenuid(
                                    imap,
                                    folder,
                                    (*(*(*imap).etpan).imap_selection_info).sel_uidvalidity,
                                    0i32 as uint32_t,
                                );
                            }
                            current_block = 17288151659885296046;
                        } else {
                            set = mailimap_set_new_single(
                                (*(*(*imap).etpan).imap_selection_info).sel_exists,
                            );
                            current_block = 11057878835866523405;
                        }
                    } else {
                        dc_log_info(
                            (*imap).context,
                            0i32,
                            b"EXISTS is missing for folder \"%s\", using fallback.\x00" as *const u8
                                as *const libc::c_char,
                            folder,
                        );
                        set = mailimap_set_new_single(0i32 as uint32_t);
                        current_block = 11057878835866523405;
                    }
                    match current_block {
                        17288151659885296046 => {}
                        _ => {
                            r = mailimap_fetch(
                                (*imap).etpan,
                                set,
                                (*imap).fetch_type_prefetch,
                                &mut fetch_result,
                            );
                            if !set.is_null() {
                                mailimap_set_free(set);
                                set = 0 as *mut mailimap_set
                            }
                            if 0 != dc_imap_is_error(imap, r) || fetch_result.is_null() {
                                fetch_result = 0 as *mut clist;
                                dc_log_info(
                                    (*imap).context,
                                    0i32,
                                    b"No result returned for folder \"%s\".\x00" as *const u8
                                        as *const libc::c_char,
                                    folder,
                                );
                                /* this might happen if the mailbox is empty an EXISTS does not work */
                                current_block = 17288151659885296046;
                            } else {
                                cur = (*fetch_result).first;
                                if cur.is_null() {
                                    dc_log_info(
                                        (*imap).context,
                                        0i32,
                                        b"Empty result returned for folder \"%s\".\x00" as *const u8
                                            as *const libc::c_char,
                                        folder,
                                    );
                                    /* this might happen if the mailbox is empty an EXISTS does not work */
                                    current_block = 17288151659885296046;
                                } else {
                                    let mut msg_att: *mut mailimap_msg_att = (if !cur.is_null() {
                                        (*cur).data
                                    } else {
                                        0 as *mut libc::c_void
                                    })
                                        as *mut mailimap_msg_att;
                                    lastseenuid = peek_uid(msg_att);
                                    if !fetch_result.is_null() {
                                        mailimap_fetch_list_free(fetch_result);
                                        fetch_result = 0 as *mut clist
                                    }
                                    if lastseenuid <= 0i32 as libc::c_uint {
                                        dc_log_error(
                                            (*imap).context,
                                            0i32,
                                            b"Cannot get largest UID for folder \"%s\"\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                            folder,
                                        );
                                        current_block = 17288151659885296046;
                                    } else {
                                        if uidvalidity > 0i32 as libc::c_uint
                                            && lastseenuid > 1i32 as libc::c_uint
                                        {
                                            lastseenuid = (lastseenuid as libc::c_uint)
                                                .wrapping_sub(1i32 as libc::c_uint)
                                                as uint32_t
                                                as uint32_t
                                        }
                                        uidvalidity =
                                            (*(*(*imap).etpan).imap_selection_info).sel_uidvalidity;
                                        set_config_lastseenuid(
                                            imap,
                                            folder,
                                            uidvalidity,
                                            lastseenuid,
                                        );
                                        dc_log_info(
                                            (*imap).context,
                                            0i32,
                                            b"lastseenuid initialized to %i for %s@%i\x00"
                                                as *const u8
                                                as *const libc::c_char,
                                            lastseenuid as libc::c_int,
                                            folder,
                                            uidvalidity as libc::c_int,
                                        );
                                        current_block = 2516253395664191498;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                current_block = 2516253395664191498;
            }
            match current_block {
                17288151659885296046 => {}
                _ => {
                    set = mailimap_set_new_interval(
                        lastseenuid.wrapping_add(1i32 as libc::c_uint),
                        0i32 as uint32_t,
                    );
                    r = mailimap_uid_fetch(
                        (*imap).etpan,
                        set,
                        (*imap).fetch_type_prefetch,
                        &mut fetch_result,
                    );
                    if !set.is_null() {
                        mailimap_set_free(set);
                        set = 0 as *mut mailimap_set
                    }
                    if 0 != dc_imap_is_error(imap, r) || fetch_result.is_null() {
                        fetch_result = 0 as *mut clist;
                        if r == MAILIMAP_ERROR_PROTOCOL as libc::c_int {
                            dc_log_info(
                                (*imap).context,
                                0i32,
                                b"Folder \"%s\" is empty\x00" as *const u8 as *const libc::c_char,
                                folder,
                            );
                        } else {
                            /* the folder is simply empty, this is no error */
                            dc_log_warning(
                                (*imap).context,
                                0i32,
                                b"Cannot fetch message list from folder \"%s\".\x00" as *const u8
                                    as *const libc::c_char,
                                folder,
                            );
                        }
                    } else {
                        cur = (*fetch_result).first;
                        while !cur.is_null() {
                            let mut msg_att_0: *mut mailimap_msg_att = (if !cur.is_null() {
                                (*cur).data
                            } else {
                                0 as *mut libc::c_void
                            })
                                as *mut mailimap_msg_att;
                            let mut cur_uid: uint32_t = peek_uid(msg_att_0);
                            if cur_uid > lastseenuid {
                                let mut rfc724_mid: *mut libc::c_char =
                                    unquote_rfc724_mid(peek_rfc724_mid(msg_att_0));
                                read_cnt = read_cnt.wrapping_add(1);
                                if 0 == (*imap).precheck_imf.expect("non-null function pointer")(
                                    imap, rfc724_mid, folder, cur_uid,
                                ) {
                                    if fetch_single_msg(imap, folder, cur_uid) == 0i32 {
                                        dc_log_info((*imap).context, 0i32,
                                                    b"Read error for message %s from \"%s\", trying over later.\x00"
                                                        as *const u8 as
                                                        *const libc::c_char,
                                                    rfc724_mid, folder);
                                        read_errors = read_errors.wrapping_add(1)
                                    }
                                } else {
                                    dc_log_info(
                                        (*imap).context,
                                        0i32,
                                        b"Skipping message %s from \"%s\" by precheck.\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                        rfc724_mid,
                                        folder,
                                    );
                                }
                                if cur_uid > new_lastseenuid {
                                    new_lastseenuid = cur_uid
                                }
                                free(rfc724_mid as *mut libc::c_void);
                            }
                            cur = if !cur.is_null() {
                                (*cur).next
                            } else {
                                0 as *mut clistcell_s
                            }
                        }
                        if 0 == read_errors && new_lastseenuid > 0i32 as libc::c_uint {
                            set_config_lastseenuid(imap, folder, uidvalidity, new_lastseenuid);
                        }
                    }
                }
            }
        }
    }
    /* done */
    if 0 != read_errors {
        dc_log_warning(
            (*imap).context,
            0i32,
            b"%i mails read from \"%s\" with %i errors.\x00" as *const u8 as *const libc::c_char,
            read_cnt as libc::c_int,
            folder,
            read_errors as libc::c_int,
        );
    } else {
        dc_log_info(
            (*imap).context,
            0i32,
            b"%i mails read from \"%s\".\x00" as *const u8 as *const libc::c_char,
            read_cnt as libc::c_int,
            folder,
        );
    }
    if !fetch_result.is_null() {
        mailimap_fetch_list_free(fetch_result);
        fetch_result = 0 as *mut clist
    }
    return read_cnt as libc::c_int;
}
unsafe extern "C" fn set_config_lastseenuid(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
    mut uidvalidity: uint32_t,
    mut lastseenuid: uint32_t,
) {
    let mut key: *mut libc::c_char = dc_mprintf(
        b"imap.mailbox.%s\x00" as *const u8 as *const libc::c_char,
        folder,
    );
    let mut val: *mut libc::c_char = dc_mprintf(
        b"%lu:%lu\x00" as *const u8 as *const libc::c_char,
        uidvalidity,
        lastseenuid,
    );
    (*imap).set_config.expect("non-null function pointer")(imap, key, val);
    free(val as *mut libc::c_void);
    free(key as *mut libc::c_void);
}
unsafe extern "C" fn peek_rfc724_mid(mut msg_att: *mut mailimap_msg_att) -> *const libc::c_char {
    if msg_att.is_null() {
        return 0 as *const libc::c_char;
    }
    /* search the UID in a list of attributes returned by a FETCH command */
    let mut iter1: *mut clistiter = 0 as *mut clistiter;
    iter1 = (*(*msg_att).att_list).first;
    while !iter1.is_null() {
        let mut item: *mut mailimap_msg_att_item = (if !iter1.is_null() {
            (*iter1).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailimap_msg_att_item;
        if !item.is_null() {
            if (*item).att_type == MAILIMAP_MSG_ATT_ITEM_STATIC as libc::c_int {
                if (*(*item).att_data.att_static).att_type
                    == MAILIMAP_MSG_ATT_ENVELOPE as libc::c_int
                {
                    let mut env: *mut mailimap_envelope =
                        (*(*item).att_data.att_static).att_data.att_env;
                    if !env.is_null() && !(*env).env_message_id.is_null() {
                        return (*env).env_message_id;
                    }
                }
            }
        }
        iter1 = if !iter1.is_null() {
            (*iter1).next
        } else {
            0 as *mut clistcell_s
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn unquote_rfc724_mid(mut in_0: *const libc::c_char) -> *mut libc::c_char {
    /* remove < and > from the given message id */
    let mut out: *mut libc::c_char = dc_strdup(in_0);
    let mut out_len: libc::c_int = strlen(out) as libc::c_int;
    if out_len > 2i32 {
        if *out.offset(0isize) as libc::c_int == '<' as i32 {
            *out.offset(0isize) = ' ' as i32 as libc::c_char
        }
        if *out.offset((out_len - 1i32) as isize) as libc::c_int == '>' as i32 {
            *out.offset((out_len - 1i32) as isize) = ' ' as i32 as libc::c_char
        }
        dc_trim(out);
    }
    return out;
}
/* ******************************************************************************
 * Fetch Messages
 ******************************************************************************/
unsafe extern "C" fn peek_uid(mut msg_att: *mut mailimap_msg_att) -> uint32_t {
    /* search the UID in a list of attributes returned by a FETCH command */
    let mut iter1: *mut clistiter = 0 as *mut clistiter;
    iter1 = (*(*msg_att).att_list).first;
    while !iter1.is_null() {
        let mut item: *mut mailimap_msg_att_item = (if !iter1.is_null() {
            (*iter1).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailimap_msg_att_item;
        if !item.is_null() {
            if (*item).att_type == MAILIMAP_MSG_ATT_ITEM_STATIC as libc::c_int {
                if (*(*item).att_data.att_static).att_type == MAILIMAP_MSG_ATT_UID as libc::c_int {
                    return (*(*item).att_data.att_static).att_data.att_uid;
                }
            }
        }
        iter1 = if !iter1.is_null() {
            (*iter1).next
        } else {
            0 as *mut clistcell_s
        }
    }
    return 0i32 as uint32_t;
}
unsafe extern "C" fn fetch_single_msg(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
    mut server_uid: uint32_t,
) -> libc::c_int {
    let mut msg_att: *mut mailimap_msg_att = 0 as *mut mailimap_msg_att;
    /* the function returns:
        0  the caller should try over again later
    or  1  if the messages should be treated as received, the caller should not try to read the message again (even if no database entries are returned) */
    let mut msg_content: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msg_bytes: size_t = 0i32 as size_t;
    let mut r: libc::c_int = 0i32;
    let mut retry_later: libc::c_int = 0i32;
    let mut deleted: libc::c_int = 0i32;
    let mut flags: uint32_t = 0i32 as uint32_t;
    let mut fetch_result: *mut clist = 0 as *mut clist;
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    if !imap.is_null() {
        if !(*imap).etpan.is_null() {
            let mut set: *mut mailimap_set = mailimap_set_new_single(server_uid);
            r = mailimap_uid_fetch(
                (*imap).etpan,
                set,
                (*imap).fetch_type_body,
                &mut fetch_result,
            );
            if !set.is_null() {
                mailimap_set_free(set);
                set = 0 as *mut mailimap_set
            }
            if 0 != dc_imap_is_error(imap, r) || fetch_result.is_null() {
                fetch_result = 0 as *mut clist;
                dc_log_warning(
                    (*imap).context,
                    0i32,
                    b"Error #%i on fetching message #%i from folder \"%s\"; retry=%i.\x00"
                        as *const u8 as *const libc::c_char,
                    r as libc::c_int,
                    server_uid as libc::c_int,
                    folder,
                    (*imap).should_reconnect as libc::c_int,
                );
                if 0 != (*imap).should_reconnect {
                    retry_later = 1i32
                }
            } else {
                /* this is an error that should be recovered; the caller should try over later to fetch the message again (if there is no such message, we simply get an empty result) */
                cur = (*fetch_result).first;
                if cur.is_null() {
                    dc_log_warning(
                        (*imap).context,
                        0i32,
                        b"Message #%i does not exist in folder \"%s\".\x00" as *const u8
                            as *const libc::c_char,
                        server_uid as libc::c_int,
                        folder,
                    );
                } else {
                    /* server response is fine, however, there is no such message, do not try to fetch the message again */
                    msg_att = (if !cur.is_null() {
                        (*cur).data
                    } else {
                        0 as *mut libc::c_void
                    }) as *mut mailimap_msg_att;
                    peek_body(
                        msg_att,
                        &mut msg_content,
                        &mut msg_bytes,
                        &mut flags,
                        &mut deleted,
                    );
                    if !(msg_content.is_null()
                        || msg_bytes <= 0i32 as libc::c_ulong
                        || 0 != deleted)
                    {
                        /* dc_log_warning(imap->context, 0, "Message #%i in folder \"%s\" is empty or deleted.", (int)server_uid, folder); -- this is a quite usual situation, do not print a warning */
                        (*imap).receive_imf.expect("non-null function pointer")(
                            imap,
                            msg_content,
                            msg_bytes,
                            folder,
                            server_uid,
                            flags,
                        );
                    }
                }
            }
        }
    }
    if !fetch_result.is_null() {
        mailimap_fetch_list_free(fetch_result);
        fetch_result = 0 as *mut clist
    }
    return if 0 != retry_later { 0i32 } else { 1i32 };
}
unsafe extern "C" fn peek_body(
    mut msg_att: *mut mailimap_msg_att,
    mut p_msg: *mut *mut libc::c_char,
    mut p_msg_bytes: *mut size_t,
    mut flags: *mut uint32_t,
    mut deleted: *mut libc::c_int,
) {
    if msg_att.is_null() {
        return;
    }
    /* search body & Co. in a list of attributes returned by a FETCH command */
    let mut iter1: *mut clistiter = 0 as *mut clistiter;
    let mut iter2: *mut clistiter = 0 as *mut clistiter;
    iter1 = (*(*msg_att).att_list).first;
    while !iter1.is_null() {
        let mut item: *mut mailimap_msg_att_item = (if !iter1.is_null() {
            (*iter1).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailimap_msg_att_item;
        if !item.is_null() {
            if (*item).att_type == MAILIMAP_MSG_ATT_ITEM_DYNAMIC as libc::c_int {
                if !(*(*item).att_data.att_dyn).att_list.is_null() {
                    iter2 = (*(*(*item).att_data.att_dyn).att_list).first;
                    while !iter2.is_null() {
                        let mut flag_fetch: *mut mailimap_flag_fetch = (if !iter2.is_null() {
                            (*iter2).data
                        } else {
                            0 as *mut libc::c_void
                        })
                            as *mut mailimap_flag_fetch;
                        if !flag_fetch.is_null()
                            && (*flag_fetch).fl_type == MAILIMAP_FLAG_FETCH_OTHER as libc::c_int
                        {
                            let mut flag: *mut mailimap_flag = (*flag_fetch).fl_flag;
                            if !flag.is_null() {
                                if (*flag).fl_type == MAILIMAP_FLAG_SEEN as libc::c_int {
                                    *flags = (*flags as libc::c_long | 0x1i64) as uint32_t
                                } else if (*flag).fl_type == MAILIMAP_FLAG_DELETED as libc::c_int {
                                    *deleted = 1i32
                                }
                            }
                        }
                        iter2 = if !iter2.is_null() {
                            (*iter2).next
                        } else {
                            0 as *mut clistcell_s
                        }
                    }
                }
            } else if (*item).att_type == MAILIMAP_MSG_ATT_ITEM_STATIC as libc::c_int {
                if (*(*item).att_data.att_static).att_type
                    == MAILIMAP_MSG_ATT_BODY_SECTION as libc::c_int
                {
                    *p_msg =
                        (*(*(*item).att_data.att_static).att_data.att_body_section).sec_body_part;
                    *p_msg_bytes =
                        (*(*(*item).att_data.att_static).att_data.att_body_section).sec_length
                }
            }
        }
        iter1 = if !iter1.is_null() {
            (*iter1).next
        } else {
            0 as *mut clistcell_s
        }
    }
}
unsafe extern "C" fn get_config_lastseenuid(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
    mut uidvalidity: *mut uint32_t,
    mut lastseenuid: *mut uint32_t,
) {
    *uidvalidity = 0i32 as uint32_t;
    *lastseenuid = 0i32 as uint32_t;
    let mut key: *mut libc::c_char = dc_mprintf(
        b"imap.mailbox.%s\x00" as *const u8 as *const libc::c_char,
        folder,
    );
    let mut val1: *mut libc::c_char =
        (*imap).get_config.expect("non-null function pointer")(imap, key, 0 as *const libc::c_char);
    let mut val2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val3: *mut libc::c_char = 0 as *mut libc::c_char;
    if !val1.is_null() {
        val2 = strchr(val1, ':' as i32);
        if !val2.is_null() {
            *val2 = 0i32 as libc::c_char;
            val2 = val2.offset(1isize);
            val3 = strchr(val2, ':' as i32);
            if !val3.is_null() {
                *val3 = 0i32 as libc::c_char
            }
            *uidvalidity = atol(val1) as uint32_t;
            *lastseenuid = atol(val2) as uint32_t
        }
    }
    free(val1 as *mut libc::c_void);
    free(key as *mut libc::c_void);
}
/* ******************************************************************************
 * Handle folders
 ******************************************************************************/
unsafe extern "C" fn select_folder(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
) -> libc::c_int {
    if imap.is_null() {
        return 0i32;
    }
    if (*imap).etpan.is_null() {
        *(*imap).selected_folder.offset(0isize) = 0i32 as libc::c_char;
        (*imap).selected_folder_needs_expunge = 0i32;
        return 0i32;
    }
    if !folder.is_null()
        && 0 != *folder.offset(0isize) as libc::c_int
        && strcmp((*imap).selected_folder, folder) == 0i32
    {
        return 1i32;
    }
    if 0 != (*imap).selected_folder_needs_expunge {
        if 0 != *(*imap).selected_folder.offset(0isize) {
            dc_log_info(
                (*imap).context,
                0i32,
                b"Expunge messages in \"%s\".\x00" as *const u8 as *const libc::c_char,
                (*imap).selected_folder,
            );
            mailimap_close((*imap).etpan);
        }
        (*imap).selected_folder_needs_expunge = 0i32
    }
    if !folder.is_null() {
        let mut r: libc::c_int = mailimap_select((*imap).etpan, folder);
        if 0 != dc_imap_is_error(imap, r) || (*(*imap).etpan).imap_selection_info.is_null() {
            dc_log_info(
                (*imap).context,
                0i32,
                b"Cannot select folder; code=%i, imap_response=%s\x00" as *const u8
                    as *const libc::c_char,
                r,
                if !(*(*imap).etpan).imap_response.is_null() {
                    (*(*imap).etpan).imap_response
                } else {
                    b"<none>\x00" as *const u8 as *const libc::c_char
                },
            );
            *(*imap).selected_folder.offset(0isize) = 0i32 as libc::c_char;
            return 0i32;
        }
    }
    free((*imap).selected_folder as *mut libc::c_void);
    (*imap).selected_folder = dc_strdup(folder);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_idle(mut imap: *mut dc_imap_t) {
    let mut current_block: u64;
    let mut r: libc::c_int = 0i32;
    let mut r2: libc::c_int = 0i32;
    if !imap.is_null() {
        if 0 != (*imap).can_idle {
            setup_handle_if_needed(imap);
            if (*imap).idle_set_up == 0i32
                && !(*imap).etpan.is_null()
                && !(*(*imap).etpan).imap_stream.is_null()
            {
                r = mailstream_setup_idle((*(*imap).etpan).imap_stream);
                if 0 != dc_imap_is_error(imap, r) {
                    dc_log_warning(
                        (*imap).context,
                        0i32,
                        b"IMAP-IDLE: Cannot setup.\x00" as *const u8 as *const libc::c_char,
                    );
                    fake_idle(imap);
                    current_block = 14832935472441733737;
                } else {
                    (*imap).idle_set_up = 1i32;
                    current_block = 17965632435239708295;
                }
            } else {
                current_block = 17965632435239708295;
            }
            match current_block {
                14832935472441733737 => {}
                _ => {
                    if 0 == (*imap).idle_set_up || 0 == select_folder(imap, (*imap).watch_folder) {
                        dc_log_warning(
                            (*imap).context,
                            0i32,
                            b"IMAP-IDLE not setup.\x00" as *const u8 as *const libc::c_char,
                        );
                        fake_idle(imap);
                    } else {
                        r = mailimap_idle((*imap).etpan);
                        if 0 != dc_imap_is_error(imap, r) {
                            dc_log_warning(
                                (*imap).context,
                                0i32,
                                b"IMAP-IDLE: Cannot start.\x00" as *const u8 as *const libc::c_char,
                            );
                            fake_idle(imap);
                        } else {
                            r = mailstream_wait_idle((*(*imap).etpan).imap_stream, 23i32 * 60i32);
                            r2 = mailimap_idle_done((*imap).etpan);
                            if r == MAILSTREAM_IDLE_ERROR as libc::c_int
                                || r == MAILSTREAM_IDLE_CANCELLED as libc::c_int
                            {
                                dc_log_info((*imap).context, 0i32,
                                            b"IMAP-IDLE wait cancelled, r=%i, r2=%i; we\'ll reconnect soon.\x00"
                                                as *const u8 as
                                                *const libc::c_char, r, r2);
                                (*imap).should_reconnect = 1i32
                            } else if r == MAILSTREAM_IDLE_INTERRUPTED as libc::c_int {
                                dc_log_info(
                                    (*imap).context,
                                    0i32,
                                    b"IMAP-IDLE interrupted.\x00" as *const u8
                                        as *const libc::c_char,
                                );
                            } else if r == MAILSTREAM_IDLE_HASDATA as libc::c_int {
                                dc_log_info(
                                    (*imap).context,
                                    0i32,
                                    b"IMAP-IDLE has data.\x00" as *const u8 as *const libc::c_char,
                                );
                            } else if r == MAILSTREAM_IDLE_TIMEOUT as libc::c_int {
                                dc_log_info(
                                    (*imap).context,
                                    0i32,
                                    b"IMAP-IDLE timeout.\x00" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                dc_log_warning(
                                    (*imap).context,
                                    0i32,
                                    b"IMAP-IDLE returns unknown value r=%i, r2=%i.\x00" as *const u8
                                        as *const libc::c_char,
                                    r,
                                    r2,
                                );
                            }
                        }
                    }
                }
            }
        } else {
            fake_idle(imap);
        }
    };
}
unsafe extern "C" fn fake_idle(mut imap: *mut dc_imap_t) {
    /* Idle using timeouts. This is also needed if we're not yet configured -
    in this case, we're waiting for a configure job */
    let mut fake_idle_start_time: time_t = time(0 as *mut time_t);
    let mut seconds_to_wait: time_t = 0i32 as time_t;
    dc_log_info(
        (*imap).context,
        0i32,
        b"IMAP-fake-IDLEing...\x00" as *const u8 as *const libc::c_char,
    );
    let mut do_fake_idle: libc::c_int = 1i32;
    while 0 != do_fake_idle {
        seconds_to_wait =
            (if time(0 as *mut time_t) - fake_idle_start_time < (3i32 * 60i32) as libc::c_long {
                5i32
            } else {
                60i32
            }) as time_t;
        pthread_mutex_lock(&mut (*imap).watch_condmutex);
        let mut r: libc::c_int = 0i32;
        let mut wakeup_at: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        memset(
            &mut wakeup_at as *mut timespec as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<timespec>() as libc::c_ulong,
        );
        wakeup_at.tv_sec = time(0 as *mut time_t) + seconds_to_wait;
        while (*imap).watch_condflag == 0i32 && r == 0i32 {
            r = pthread_cond_timedwait(
                &mut (*imap).watch_cond,
                &mut (*imap).watch_condmutex,
                &mut wakeup_at,
            );
            if 0 != (*imap).watch_condflag {
                do_fake_idle = 0i32
            }
        }
        (*imap).watch_condflag = 0i32;
        pthread_mutex_unlock(&mut (*imap).watch_condmutex);
        if do_fake_idle == 0i32 {
            return;
        }
        if 0 != setup_handle_if_needed(imap) {
            if 0 != fetch_from_single_folder(imap, (*imap).watch_folder) {
                do_fake_idle = 0i32
            }
        } else {
            fake_idle_start_time = 0i32 as time_t
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_interrupt_idle(mut imap: *mut dc_imap_t) {
    if imap.is_null() {
        return;
    }
    if 0 != (*imap).can_idle {
        if !(*imap).etpan.is_null() && !(*(*imap).etpan).imap_stream.is_null() {
            mailstream_interrupt_idle((*(*imap).etpan).imap_stream);
        }
    }
    pthread_mutex_lock(&mut (*imap).watch_condmutex);
    (*imap).watch_condflag = 1i32;
    pthread_cond_signal(&mut (*imap).watch_cond);
    pthread_mutex_unlock(&mut (*imap).watch_condmutex);
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_move(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
    mut uid: uint32_t,
    mut dest_folder: *const libc::c_char,
    mut dest_uid: *mut uint32_t,
) -> dc_imap_res {
    let mut current_block: u64;
    let mut res: dc_imap_res = DC_RETRY_LATER;
    let mut r: libc::c_int = 0i32;
    let mut set: *mut mailimap_set = mailimap_set_new_single(uid);
    let mut res_uid: uint32_t = 0i32 as uint32_t;
    let mut res_setsrc: *mut mailimap_set = 0 as *mut mailimap_set;
    let mut res_setdest: *mut mailimap_set = 0 as *mut mailimap_set;
    if imap.is_null()
        || folder.is_null()
        || uid == 0i32 as libc::c_uint
        || dest_folder.is_null()
        || dest_uid.is_null()
        || set.is_null()
    {
        res = DC_FAILED
    } else if strcasecmp(folder, dest_folder) == 0i32 {
        dc_log_info(
            (*imap).context,
            0i32,
            b"Skip moving message; message %s/%i is already in %s...\x00" as *const u8
                as *const libc::c_char,
            folder,
            uid as libc::c_int,
            dest_folder,
        );
        res = DC_ALREADY_DONE
    } else {
        dc_log_info(
            (*imap).context,
            0i32,
            b"Moving message %s/%i to %s...\x00" as *const u8 as *const libc::c_char,
            folder,
            uid as libc::c_int,
            dest_folder,
        );
        if select_folder(imap, folder) == 0i32 {
            dc_log_warning(
                (*imap).context,
                0i32,
                b"Cannot select folder %s for moving message.\x00" as *const u8
                    as *const libc::c_char,
                folder,
            );
        } else {
            r = mailimap_uidplus_uid_move(
                (*imap).etpan,
                set,
                dest_folder,
                &mut res_uid,
                &mut res_setsrc,
                &mut res_setdest,
            );
            if 0 != dc_imap_is_error(imap, r) {
                if !res_setsrc.is_null() {
                    mailimap_set_free(res_setsrc);
                    res_setsrc = 0 as *mut mailimap_set
                }
                if !res_setdest.is_null() {
                    mailimap_set_free(res_setdest);
                    res_setdest = 0 as *mut mailimap_set
                }
                dc_log_info(
                    (*imap).context,
                    0i32,
                    b"Cannot move message, fallback to COPY/DELETE %s/%i to %s...\x00" as *const u8
                        as *const libc::c_char,
                    folder,
                    uid as libc::c_int,
                    dest_folder,
                );
                r = mailimap_uidplus_uid_copy(
                    (*imap).etpan,
                    set,
                    dest_folder,
                    &mut res_uid,
                    &mut res_setsrc,
                    &mut res_setdest,
                );
                if 0 != dc_imap_is_error(imap, r) {
                    dc_log_info(
                        (*imap).context,
                        0i32,
                        b"Cannot copy message.\x00" as *const u8 as *const libc::c_char,
                    );
                    current_block = 14415637129417834392;
                } else {
                    if add_flag(imap, uid, mailimap_flag_new_deleted()) == 0i32 {
                        dc_log_warning(
                            (*imap).context,
                            0i32,
                            b"Cannot mark message as \"Deleted\".\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    (*imap).selected_folder_needs_expunge = 1i32;
                    current_block = 1538046216550696469;
                }
            } else {
                current_block = 1538046216550696469;
            }
            match current_block {
                14415637129417834392 => {}
                _ => {
                    if !res_setdest.is_null() {
                        let mut cur: *mut clistiter = (*(*res_setdest).set_list).first;
                        if !cur.is_null() {
                            let mut item: *mut mailimap_set_item = 0 as *mut mailimap_set_item;
                            item = (if !cur.is_null() {
                                (*cur).data
                            } else {
                                0 as *mut libc::c_void
                            }) as *mut mailimap_set_item;
                            *dest_uid = (*item).set_first
                        }
                    }
                    res = DC_SUCCESS
                }
            }
        }
    }
    if !set.is_null() {
        mailimap_set_free(set);
        set = 0 as *mut mailimap_set
    }
    if !res_setsrc.is_null() {
        mailimap_set_free(res_setsrc);
        res_setsrc = 0 as *mut mailimap_set
    }
    if !res_setdest.is_null() {
        mailimap_set_free(res_setdest);
        res_setdest = 0 as *mut mailimap_set
    }
    return (if res as libc::c_uint == DC_RETRY_LATER as libc::c_int as libc::c_uint {
        (if 0 != (*imap).should_reconnect {
            DC_RETRY_LATER as libc::c_int
        } else {
            DC_FAILED as libc::c_int
        }) as libc::c_uint
    } else {
        res as libc::c_uint
    }) as dc_imap_res;
}
unsafe extern "C" fn add_flag(
    mut imap: *mut dc_imap_t,
    mut server_uid: uint32_t,
    mut flag: *mut mailimap_flag,
) -> libc::c_int {
    let mut r: libc::c_int = 0i32;
    let mut flag_list: *mut mailimap_flag_list = 0 as *mut mailimap_flag_list;
    let mut store_att_flags: *mut mailimap_store_att_flags = 0 as *mut mailimap_store_att_flags;
    let mut set: *mut mailimap_set = mailimap_set_new_single(server_uid);
    if !(imap.is_null() || (*imap).etpan.is_null()) {
        flag_list = mailimap_flag_list_new_empty();
        mailimap_flag_list_add(flag_list, flag);
        store_att_flags = mailimap_store_att_flags_new_add_flags(flag_list);
        r = mailimap_uid_store((*imap).etpan, set, store_att_flags);
        0 != dc_imap_is_error(imap, r);
    }
    if !store_att_flags.is_null() {
        mailimap_store_att_flags_free(store_att_flags);
    }
    if !set.is_null() {
        mailimap_set_free(set);
        set = 0 as *mut mailimap_set
    }
    return if 0 != (*imap).should_reconnect {
        0i32
    } else {
        1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_set_seen(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
    mut uid: uint32_t,
) -> dc_imap_res {
    let mut res: dc_imap_res = DC_RETRY_LATER;
    if imap.is_null() || folder.is_null() || uid == 0i32 as libc::c_uint {
        res = DC_FAILED
    } else if !(*imap).etpan.is_null() {
        dc_log_info(
            (*imap).context,
            0i32,
            b"Marking message %s/%i as seen...\x00" as *const u8 as *const libc::c_char,
            folder,
            uid as libc::c_int,
        );
        if select_folder(imap, folder) == 0i32 {
            dc_log_warning(
                (*imap).context,
                0i32,
                b"Cannot select folder %s for setting SEEN flag.\x00" as *const u8
                    as *const libc::c_char,
                folder,
            );
        } else if add_flag(imap, uid, mailimap_flag_new_seen()) == 0i32 {
            dc_log_warning(
                (*imap).context,
                0i32,
                b"Cannot mark message as seen.\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            res = DC_SUCCESS
        }
    }
    return (if res as libc::c_uint == DC_RETRY_LATER as libc::c_int as libc::c_uint {
        (if 0 != (*imap).should_reconnect {
            DC_RETRY_LATER as libc::c_int
        } else {
            DC_FAILED as libc::c_int
        }) as libc::c_uint
    } else {
        res as libc::c_uint
    }) as dc_imap_res;
}
#[no_mangle]
pub unsafe extern "C" fn dc_imap_set_mdnsent(
    mut imap: *mut dc_imap_t,
    mut folder: *const libc::c_char,
    mut uid: uint32_t,
) -> dc_imap_res {
    let mut can_create_flag: libc::c_int = 0;
    let mut current_block: u64;
    // returns 0=job should be retried later, 1=job done, 2=job done and flag just set
    let mut res: dc_imap_res = DC_RETRY_LATER;
    let mut set: *mut mailimap_set = mailimap_set_new_single(uid);
    let mut fetch_result: *mut clist = 0 as *mut clist;
    if imap.is_null() || folder.is_null() || uid == 0i32 as libc::c_uint || set.is_null() {
        res = DC_FAILED
    } else if !(*imap).etpan.is_null() {
        dc_log_info(
            (*imap).context,
            0i32,
            b"Marking message %s/%i as $MDNSent...\x00" as *const u8 as *const libc::c_char,
            folder,
            uid as libc::c_int,
        );
        if select_folder(imap, folder) == 0i32 {
            dc_log_warning(
                (*imap).context,
                0i32,
                b"Cannot select folder %s for setting $MDNSent flag.\x00" as *const u8
                    as *const libc::c_char,
                folder,
            );
        } else {
            /* Check if the folder can handle the `$MDNSent` flag (see RFC 3503).  If so, and not set: set the flags and return this information.
            If the folder cannot handle the `$MDNSent` flag, we risk duplicated MDNs; it's up to the receiving MUA to handle this then (eg. Delta Chat has no problem with this). */
            can_create_flag = 0i32;
            if !(*(*imap).etpan).imap_selection_info.is_null()
                && !(*(*(*imap).etpan).imap_selection_info)
                    .sel_perm_flags
                    .is_null()
            {
                let mut iter: *mut clistiter = 0 as *mut clistiter;
                iter = (*(*(*(*imap).etpan).imap_selection_info).sel_perm_flags).first;
                while !iter.is_null() {
                    let mut fp: *mut mailimap_flag_perm = (if !iter.is_null() {
                        (*iter).data
                    } else {
                        0 as *mut libc::c_void
                    })
                        as *mut mailimap_flag_perm;
                    if !fp.is_null() {
                        if (*fp).fl_type == MAILIMAP_FLAG_PERM_ALL as libc::c_int {
                            can_create_flag = 1i32;
                            break;
                        } else if (*fp).fl_type == MAILIMAP_FLAG_PERM_FLAG as libc::c_int
                            && !(*fp).fl_flag.is_null()
                        {
                            let mut fl: *mut mailimap_flag = (*fp).fl_flag as *mut mailimap_flag;
                            if (*fl).fl_type == MAILIMAP_FLAG_KEYWORD as libc::c_int
                                && !(*fl).fl_data.fl_keyword.is_null()
                                && strcmp(
                                    (*fl).fl_data.fl_keyword,
                                    b"$MDNSent\x00" as *const u8 as *const libc::c_char,
                                ) == 0i32
                            {
                                can_create_flag = 1i32;
                                break;
                            }
                        }
                    }
                    iter = if !iter.is_null() {
                        (*iter).next
                    } else {
                        0 as *mut clistcell_s
                    }
                }
            }
            if 0 != can_create_flag {
                let mut r: libc::c_int = mailimap_uid_fetch(
                    (*imap).etpan,
                    set,
                    (*imap).fetch_type_flags,
                    &mut fetch_result,
                );
                if 0 != dc_imap_is_error(imap, r) || fetch_result.is_null() {
                    fetch_result = 0 as *mut clist
                } else {
                    let mut cur: *mut clistiter = (*fetch_result).first;
                    if !cur.is_null() {
                        if 0 != peek_flag_keyword(
                            (if !cur.is_null() {
                                (*cur).data
                            } else {
                                0 as *mut libc::c_void
                            }) as *mut mailimap_msg_att,
                            b"$MDNSent\x00" as *const u8 as *const libc::c_char,
                        ) {
                            res = DC_ALREADY_DONE;
                            current_block = 14832935472441733737;
                        } else if add_flag(
                            imap,
                            uid,
                            mailimap_flag_new_flag_keyword(dc_strdup(
                                b"$MDNSent\x00" as *const u8 as *const libc::c_char,
                            )),
                        ) == 0i32
                        {
                            current_block = 17044610252497760460;
                        } else {
                            res = DC_SUCCESS;
                            current_block = 14832935472441733737;
                        }
                        match current_block {
                            17044610252497760460 => {}
                            _ => {
                                dc_log_info(
                                    (*imap).context,
                                    0i32,
                                    if res as libc::c_uint
                                        == DC_SUCCESS as libc::c_int as libc::c_uint
                                    {
                                        b"$MDNSent just set and MDN will be sent.\x00" as *const u8
                                            as *const libc::c_char
                                    } else {
                                        b"$MDNSent already set and MDN already sent.\x00"
                                            as *const u8
                                            as *const libc::c_char
                                    },
                                );
                            }
                        }
                    }
                }
            } else {
                res = DC_SUCCESS;
                dc_log_info(
                    (*imap).context,
                    0i32,
                    b"Cannot store $MDNSent flags, risk sending duplicate MDN.\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        }
    }
    if !set.is_null() {
        mailimap_set_free(set);
        set = 0 as *mut mailimap_set
    }
    if !fetch_result.is_null() {
        mailimap_fetch_list_free(fetch_result);
        fetch_result = 0 as *mut clist
    }
    return (if res as libc::c_uint == DC_RETRY_LATER as libc::c_int as libc::c_uint {
        (if 0 != (*imap).should_reconnect {
            DC_RETRY_LATER as libc::c_int
        } else {
            DC_FAILED as libc::c_int
        }) as libc::c_uint
    } else {
        res as libc::c_uint
    }) as dc_imap_res;
}
unsafe extern "C" fn peek_flag_keyword(
    mut msg_att: *mut mailimap_msg_att,
    mut flag_keyword: *const libc::c_char,
) -> libc::c_int {
    if msg_att.is_null() || (*msg_att).att_list.is_null() || flag_keyword.is_null() {
        return 0i32;
    }
    let mut iter1: *mut clistiter = 0 as *mut clistiter;
    let mut iter2: *mut clistiter = 0 as *mut clistiter;
    iter1 = (*(*msg_att).att_list).first;
    while !iter1.is_null() {
        let mut item: *mut mailimap_msg_att_item = (if !iter1.is_null() {
            (*iter1).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailimap_msg_att_item;
        if !item.is_null() {
            if (*item).att_type == MAILIMAP_MSG_ATT_ITEM_DYNAMIC as libc::c_int {
                if !(*(*item).att_data.att_dyn).att_list.is_null() {
                    iter2 = (*(*(*item).att_data.att_dyn).att_list).first;
                    while !iter2.is_null() {
                        let mut flag_fetch: *mut mailimap_flag_fetch = (if !iter2.is_null() {
                            (*iter2).data
                        } else {
                            0 as *mut libc::c_void
                        })
                            as *mut mailimap_flag_fetch;
                        if !flag_fetch.is_null()
                            && (*flag_fetch).fl_type == MAILIMAP_FLAG_FETCH_OTHER as libc::c_int
                        {
                            let mut flag: *mut mailimap_flag = (*flag_fetch).fl_flag;
                            if !flag.is_null() {
                                if (*flag).fl_type == MAILIMAP_FLAG_KEYWORD as libc::c_int
                                    && !(*flag).fl_data.fl_keyword.is_null()
                                    && strcmp((*flag).fl_data.fl_keyword, flag_keyword) == 0i32
                                {
                                    return 1i32;
                                }
                            }
                        }
                        iter2 = if !iter2.is_null() {
                            (*iter2).next
                        } else {
                            0 as *mut clistcell_s
                        }
                    }
                }
            }
        }
        iter1 = if !iter1.is_null() {
            (*iter1).next
        } else {
            0 as *mut clistcell_s
        }
    }
    return 0i32;
}
/* only returns 0 on connection problems; we should try later again in this case */
#[no_mangle]
pub unsafe extern "C" fn dc_imap_delete_msg(
    mut imap: *mut dc_imap_t,
    mut rfc724_mid: *const libc::c_char,
    mut folder: *const libc::c_char,
    mut server_uid: uint32_t,
) -> libc::c_int {
    let mut success: libc::c_int = 0i32;
    let mut r: libc::c_int = 0i32;
    let mut fetch_result: *mut clist = 0 as *mut clist;
    let mut is_rfc724_mid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_folder: *mut libc::c_char = 0 as *mut libc::c_char;
    if imap.is_null()
        || rfc724_mid.is_null()
        || folder.is_null()
        || *folder.offset(0isize) as libc::c_int == 0i32
        || server_uid == 0i32 as libc::c_uint
    {
        success = 1i32
    } else {
        dc_log_info(
            (*imap).context,
            0i32,
            b"Marking message \"%s\", %s/%i for deletion...\x00" as *const u8
                as *const libc::c_char,
            rfc724_mid,
            folder,
            server_uid as libc::c_int,
        );
        if select_folder(imap, folder) == 0i32 {
            dc_log_warning(
                (*imap).context,
                0i32,
                b"Cannot select folder %s for deleting message.\x00" as *const u8
                    as *const libc::c_char,
                folder,
            );
        } else {
            let mut cur: *mut clistiter = 0 as *mut clistiter;
            let mut is_quoted_rfc724_mid: *const libc::c_char = 0 as *const libc::c_char;
            let mut set: *mut mailimap_set = mailimap_set_new_single(server_uid);
            r = mailimap_uid_fetch(
                (*imap).etpan,
                set,
                (*imap).fetch_type_prefetch,
                &mut fetch_result,
            );
            if !set.is_null() {
                mailimap_set_free(set);
                set = 0 as *mut mailimap_set
            }
            if 0 != dc_imap_is_error(imap, r) || fetch_result.is_null() {
                fetch_result = 0 as *mut clist;
                dc_log_warning(
                    (*imap).context,
                    0i32,
                    b"Cannot delete on IMAP, %s/%i not found.\x00" as *const u8
                        as *const libc::c_char,
                    folder,
                    server_uid as libc::c_int,
                );
                server_uid = 0i32 as uint32_t
            }
            cur = (*fetch_result).first;
            if cur.is_null()
                || {
                    is_quoted_rfc724_mid = peek_rfc724_mid(
                        (if !cur.is_null() {
                            (*cur).data
                        } else {
                            0 as *mut libc::c_void
                        }) as *mut mailimap_msg_att,
                    );
                    is_quoted_rfc724_mid.is_null()
                }
                || {
                    is_rfc724_mid = unquote_rfc724_mid(is_quoted_rfc724_mid);
                    is_rfc724_mid.is_null()
                }
                || strcmp(is_rfc724_mid, rfc724_mid) != 0i32
            {
                dc_log_warning(
                    (*imap).context,
                    0i32,
                    b"Cannot delete on IMAP, %s/%i does not match %s.\x00" as *const u8
                        as *const libc::c_char,
                    folder,
                    server_uid as libc::c_int,
                    rfc724_mid,
                );
                server_uid = 0i32 as uint32_t
            }
            /* mark the message for deletion */
            if add_flag(imap, server_uid, mailimap_flag_new_deleted()) == 0i32 {
                dc_log_warning(
                    (*imap).context,
                    0i32,
                    b"Cannot mark message as \"Deleted\".\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                (*imap).selected_folder_needs_expunge = 1i32;
                success = 1i32
            }
        }
    }
    if !fetch_result.is_null() {
        mailimap_fetch_list_free(fetch_result);
        fetch_result = 0 as *mut clist
    }
    free(is_rfc724_mid as *mut libc::c_void);
    free(new_folder as *mut libc::c_void);
    return if 0 != success {
        1i32
    } else {
        dc_imap_is_connected(imap)
    };
}
