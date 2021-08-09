use nx::result::*;
use nx::ipc::sf;
use nx::ipc::server;
use nx::ipc::sf::sm;
use nx::diag::log;

pub trait IPrepoService {
    ipc_cmif_interface_define_command!(save_report_old: (process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(save_report_with_user_old: (user_id: u128, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(save_report_old_2: (process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(save_report_with_user_old_2: (user_id: u128, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(save_report: (process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(save_report_with_user: (user_id: u128, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(request_immediate_transmission: () => ());
    ipc_cmif_interface_define_command!(get_transmission_status: () => (status: u32));
    ipc_cmif_interface_define_command!(get_system_session_id: () => (id: u64));
    ipc_cmif_interface_define_command!(save_system_report: (application_id: u64, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
    ipc_cmif_interface_define_command!(save_system_report_with_user: (user_id: u128, application_id: u64, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) => ());
}

pub const SERVICE_TYPE_ADMIN: u32 = 1;
pub const SERVICE_TYPE_ADMIN2: u32 = 2;
pub const SERVICE_TYPE_MANAGER: u32 = 3;
pub const SERVICE_TYPE_USER: u32 = 4;
pub const SERVICE_TYPE_SYSTEM: u32 = 5;

fn get_service_name<const S: u32>() -> &'static str {
    match S {
        SERVICE_TYPE_ADMIN => nul!("prepo:a"),
        SERVICE_TYPE_ADMIN2 => nul!("prepo:a2"),
        SERVICE_TYPE_MANAGER => nul!("prepo:m"),
        SERVICE_TYPE_USER => nul!("prepo:u"),
        SERVICE_TYPE_SYSTEM => nul!("prepo:s"),
        _ => nul!("")
    }
}

fn get_non_null_service_name<const S: u32>() -> &'static str {
    get_service_name::<S>().trim_matches('\0')
}

#[derive(Debug)]
pub enum ReportKind {
    Normal,
    System
}

pub struct ReportContext {
    pub kind: ReportKind,
    pub process_id: Option<u64>,
    pub application_id: Option<u64>,
    pub room_str_buf: sf::InPointerBuffer,
    pub report_msgpack_buf: sf::InMapAliasBuffer,
    pub user_id: Option<u128>
}

pub struct PrepoService<const S: u32> {
    session: sf::Session,
    _info: sm::MitmProcessInfo
}

impl<const S: u32> PrepoService<S> {
    fn process_report(&self, ctx: ReportContext) {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "\nREPORT START\n");

        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Kind: {:?}\n", ctx.kind);
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Room: {}\n", ctx.room_str_buf.get_string());
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Msgpack size: {}\n", ctx.report_msgpack_buf.size);
        
        if let Some(process_id) = ctx.process_id {
            diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Process (ID) sending the report: {:#X}\n", process_id);
        }
        if let Some(application_id) = ctx.application_id {
            diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Application (ID) sending the report: {:#X}\n", application_id);
        }
        if let Some(_user_id) = ctx.user_id {
            let user_name = "TODOUser";
            diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "User sending the report: {}\n", user_name);
        }
        

        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "REPORT END\n");
    }

    fn save_report_impl(&self, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        let ctx = ReportContext {
            kind: ReportKind::Normal,
            process_id: Some(process_id.process_id),
            application_id: None,
            room_str_buf: room_str_buf,
            report_msgpack_buf: report_msgpack_buf,
            user_id: None
        };
        self.process_report(ctx);
        Ok(())
    }

    fn save_report_with_user_impl(&self, user_id: u128, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        let ctx = ReportContext {
            kind: ReportKind::Normal,
            process_id: Some(process_id.process_id),
            application_id: None,
            room_str_buf: room_str_buf,
            report_msgpack_buf: report_msgpack_buf,
            user_id: Some(user_id)
        };
        self.process_report(ctx);
        Ok(())
    }

    fn save_system_report_impl(&self, application_id: u64, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        let ctx = ReportContext {
            kind: ReportKind::System,
            process_id: None,
            application_id: Some(application_id),
            room_str_buf: room_str_buf,
            report_msgpack_buf: report_msgpack_buf,
            user_id: None
        };
        self.process_report(ctx);
        Ok(())
    }

    fn save_system_report_with_user_impl(&self, user_id: u128, application_id: u64, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        let ctx = ReportContext {
            kind: ReportKind::System,
            process_id: None,
            application_id: Some(application_id),
            room_str_buf: room_str_buf,
            report_msgpack_buf: report_msgpack_buf,
            user_id: Some(user_id)
        };
        self.process_report(ctx);
        Ok(())
    }
}

impl<const S: u32> sf::IObject for PrepoService<S> {
    fn get_session(&mut self) -> &mut sf::Session {
        &mut self.session
    }

    fn get_command_table(&self) -> sf::CommandMetadataTable {
        vec! [
            ipc_cmif_interface_make_command_meta!(save_report_old: 10100),
            ipc_cmif_interface_make_command_meta!(save_report_with_user_old: 10101),
            ipc_cmif_interface_make_command_meta!(save_report_old_2: 10102),
            ipc_cmif_interface_make_command_meta!(save_report_with_user_old_2: 10103),
            ipc_cmif_interface_make_command_meta!(save_report: 10104),
            ipc_cmif_interface_make_command_meta!(save_report_with_user: 10105),
            ipc_cmif_interface_make_command_meta!(request_immediate_transmission: 10200),
            ipc_cmif_interface_make_command_meta!(get_transmission_status: 10300),
            ipc_cmif_interface_make_command_meta!(get_system_session_id: 10400),
            ipc_cmif_interface_make_command_meta!(save_system_report: 20100),
            ipc_cmif_interface_make_command_meta!(save_system_report_with_user: 20101)
        ]
    }
}

impl<const S: u32> server::IMitmServerObject for PrepoService<S> {
    fn new(info: sm::MitmProcessInfo) -> Self {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Opened '{}' from program {:#X}\n", get_non_null_service_name::<S>(), info.program_id);
        Self { session: sf::Session::new(), _info: info }
    }
}

impl<const S: u32> IPrepoService for PrepoService<S> {
    fn save_report_old(&mut self, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_report_impl(process_id, room_str_buf, report_msgpack_buf)
    }

    fn save_report_with_user_old(&mut self, user_id: u128, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_report_with_user_impl(user_id, process_id, room_str_buf, report_msgpack_buf)
    }

    fn save_report_old_2(&mut self, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_report_impl(process_id, room_str_buf, report_msgpack_buf)
    }

    fn save_report_with_user_old_2(&mut self, user_id: u128, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_report_with_user_impl(user_id, process_id, room_str_buf, report_msgpack_buf)
    }

    fn save_report(&mut self, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_report_impl(process_id, room_str_buf, report_msgpack_buf)
    }

    fn save_report_with_user(&mut self, user_id: u128, process_id: sf::ProcessId, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_report_with_user_impl(user_id, process_id, room_str_buf, report_msgpack_buf)
    }

    fn request_immediate_transmission(&mut self) -> Result<()> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "\nRequesting immediate transmission...\n");
        Ok(())
    }

    fn get_transmission_status(&mut self) -> Result<u32> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "\nSending transmission status...\n");
        Ok(0)
    }

    fn get_system_session_id(&mut self) -> Result<u64> {
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "\nSending session ID...\n");
        Ok(0xBABABEBE)
    }

    fn save_system_report(&mut self, application_id: u64, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_system_report_impl(application_id, room_str_buf, report_msgpack_buf)
    }

    fn save_system_report_with_user(&mut self, user_id: u128, application_id: u64, room_str_buf: sf::InPointerBuffer, report_msgpack_buf: sf::InMapAliasBuffer) -> Result<()> {
        self.save_system_report_with_user_impl(user_id, application_id, room_str_buf, report_msgpack_buf)
    }
}

impl<const S: u32> server::IMitmService for PrepoService<S> {
    fn get_name() -> &'static str {
        let name = get_service_name::<S>();
        diag_log!(log::LmLogger { log::LogSeverity::Info, true } => "Registering mitm at service '{}'...\n", get_non_null_service_name::<S>());
        name
    }

    fn should_mitm(_info: sm::MitmProcessInfo) -> bool {
        true
    }
}