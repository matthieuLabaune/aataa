// Type definitions for Tauri API
declare module '@tauri-apps/plugin-dialog' {
    export interface DialogFilter {
        name: string
        extensions: string[]
    }

    export interface OpenDialogOptions {
        multiple?: boolean
        directory?: boolean
        filters?: DialogFilter[]
    }

    export function open(options?: OpenDialogOptions): Promise<string | string[] | null>
}
