import { open } from '@tauri-apps/plugin-dialog';

export async function pickDirectory(defaultPath: string = "~/") {
    // Open a selection dialog for directories
    const selected = await open({
        directory: true,
        multiple: false,
        defaultPath: defaultPath,
    });

    return selected
}