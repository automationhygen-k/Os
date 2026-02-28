mod os;

use os::assistant::Assistant;
use os::kernel::Kernel;
use os::security::{Capability, SecurityProfile};
use os::ui::{AestheticStyle, ExperienceTheme};

fn main() -> Result<(), String> {
    let mut kernel = Kernel::new(
        "AetherOS",
        SecurityProfile::hardened_mobile_like(),
        ExperienceTheme::new("Heaven", AestheticStyle::GlassDepth),
    );

    kernel.boot()?;
    kernel.register_capability(Capability::ControlAudio);
    kernel.register_capability(Capability::ControlDisplay);
    kernel.register_capability(Capability::ControlNetworking);
    kernel.register_capability(Capability::LaunchApps);
    kernel.register_capability(Capability::InstallApps);
    kernel.register_capability(Capability::ManageProcesses);
    kernel.register_capability(Capability::UseTerminal);
    kernel.register_capability(Capability::ManageFiles);
    kernel.register_capability(Capability::ManageDesktop);
    kernel.register_capability(Capability::ManageSettings);
    kernel.register_capability(Capability::ManageNotifications);

    let mut assistant = Assistant::new("Nova", kernel);
    assistant.seed_demo_apps();

    let script = [
        "latency 12",
        "design integrity",
        "set volume to 25",
        "set brightness 72",
        "focus deep",
        "power creator",
        "dock",
        "open window Control Center",
        "mission control",
        "files write notes/today.txt :: refinement over noise",
        "files read notes/today.txt",
        "notify low backup finished",
        "notify low sync complete",
        "flush digest",
        "notifications",
        "suggest reopen last workspace",
        "ignore suggestion reopen last workspace",
        "ignore suggestion reopen last workspace",
        "suggest reopen last workspace",
        "who are you",
        "launch Echo Demo",
        "list processes",
        "diagnostics",
    ];

    for cmd in script {
        println!("[Assistant] {}", assistant.handle_command(cmd)?);
    }

    Ok(())
}
