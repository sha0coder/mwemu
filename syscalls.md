# Esquema de syscalls implementadas en mwemu

Resumen por fichero, una frase por syscall describiendo cómo está implementada.
Raíz: `crates/libmwemu/src/syscall/`.

---

## Windows x64 — `syscall/windows/syscall64/`

### 1. `mod.rs` (dispatcher)
- **1.1. `build_syscall_translation_table`** — escanea el `ntdll.pe` cargado buscando el prólogo `4c 8b d1 b8 <imm32>` (mov r10,rcx; mov eax,imm32) y construye un mapa entre los números reales del ntdll de turno y los `WIN64_NT*` canónicos del dispatcher.
- **1.2. `gateway`** — traduce el `nr` entrante con la tabla anterior y hace `match` para enrutar cada syscall a su `nt_*` correspondiente (más de 100 entradas).
- **1.3. `what_syscall`** — utilidad inversa: devuelve el nombre PascalCase a partir del número (para logs).

### 2. `process.rs`
- **2.1. `NtAccessCheck`** — stub permisivo: escribe `DesiredAccess` en `GrantedAccess` y `STATUS_SUCCESS` en `AccessStatus` (ntdll lo usa en chequeos de seguridad del loader).
- **2.2. `NtQueryInformationProcess`** — atiende clases `ProcessBasicInformation` (rellena `PROCESS_BASIC_INFORMATION` apuntando al PEB), `ProcessCookie` (escribe cookie `0x01234567`), `ProcessDebugPort` (0 = no debugger), `ProcessWow64Information` (0 = nativo) y `ProcessImageFileName` (UNICODE_STRING vacía); el resto → `STATUS_INVALID_PARAMETER`.
- **2.3. `NtQueryPerformanceCounter`** — devuelve `emu.tick * 1000` como contador y `10_000_000` como frecuencia (QPC falso de ~10 MHz).
- **2.4. `NtQueryInformationThread`** — para `ThreadBasicInformation` escribe TEB base, PID/TID falsos `0x1000/0x1004`; para `Win32StartAddress` escribe `RIP` actual.
- **2.5. `NtSetInformationProcess`** — sólo acepta el handle del proceso actual (`-1`), devuelve `STATUS_SUCCESS` sin guardar nada.
- **2.6. `NtSetInformationThread`** — no-op; siempre `STATUS_SUCCESS`.
- **2.7. `NtOpenProcess`** — escribe handle falso `0x4` en la salida y devuelve `STATUS_SUCCESS`.
- **2.8. `NtRaiseHardError`** — vuelca los `Parameters` (resolviendo `PUNICODE_STRING` si la máscara lo indica), escribe `ResponseOk (6)` en `*Response` y devuelve éxito para que ntdll no se bloquee.
- **2.9. `NtTerminateProcess`** — si el handle es el proceso actual marca `emu.process_terminated` y detiene la emulación; si no, devuelve `STATUS_ACCESS_DENIED`.
- **2.10. `NtRaiseException`** — restaura RIP/RSP/registros generales desde el `CONTEXT` recibido y activa `force_reload` para redirigir la ejecución (en Windows nunca retorna).
- **2.11. `NtQuerySecurityAttributesToken`** — escribe `TOKEN_SECURITY_ATTRIBUTES_INFORMATION` vacío (Version=1, AttributeCount=0).
- **2.12. `NtCreateThreadEx`** — stub mono-hilo: graba un handle sintético (`sync::next_handle()`) en `*ThreadHandle` y devuelve éxito (no crea hilo real).
- **2.13. `NtContinue`** — lee el `CONTEXT` y restaura registros + RIP/RSP; usado por `LdrInitializeThunk` para saltar al entrypoint del PE.
- **2.14. `NtDuplicateObject`** — si hay `TargetHandle`, escribe un handle fresco con `sync::next_handle()`.
- **2.15. `NtCreateProfileEx`** — devuelve `STATUS_NOT_SUPPORTED`.

### 3. `memory.rs`
- **3.1. `ntdll_heap_list_walk_fixup`** — *no es una syscall*: hook de instrucción que parchea una `LIST_ENTRY` zeroed en `allocBase+0xb000` enlazándola a sí misma para que `RtlAllocateHeap` no falle al seguir el Flink.
- **3.2. `NtQueryVirtualMemory`** — describe la región preguntada usando `emu.maps` (MEMORY_BASIC_INFORMATION o nombre de la sección).
- **3.3. `NtAllocateVirtualMemory`** — reserva/commitea memoria a través del allocator de `emu.maps`, devolviendo la base y tamaño al caller.
- **3.4. `NtAllocateVirtualMemoryEx`** — variante con `MEM_EXTENDED_PARAMETER`; ignora extensiones y delega en la misma reserva alineada a 64 K.
- **3.5. `NtFreeVirtualMemory`** — libera la región de `emu.maps` correspondiente a la base solicitada.
- **3.6. `NtProtectVirtualMemory`** — cambia los permisos del mapa y devuelve los antiguos en `*OldProtect`.
- **3.7. `NtReadVirtualMemory`** — copia bytes entre regiones mapeadas y rellena `NumberOfBytesRead`.
- **3.8. `NtWriteVirtualMemory`** — análogo en sentido contrario.
- **3.9. `NtUnmapViewOfSection`** — no-op; ntdll re-mapea encima y conservar el mapa original evita romper lecturas del PE.
- **3.10. `NtMapViewOfSection`** — si el handle es de `\KnownDlls\<dll>` carga el PE real desde `cfg.maps_folder`; en otro caso reserva una región anónima del tamaño pedido.
- **3.11. `NtAllocateUserPhysicalPagesEx`** — devuelve `STATUS_PRIVILEGE_NOT_HELD` para que ntdll caiga al heap normal (no se emula AWE).
- **3.12. `NtOpenSection` / `NtCreateSection`** — escriben un handle falso; si el nombre es `\KnownDlls\*.dll` se memoriza para que el `MapView` posterior cargue el binario correcto.

### 4. `registry.rs`
- **4.1. `NtQueryMultipleValueKey`** — pone `*BufferLength = 0` y devuelve éxito (el bucle del caller no recorre nada).
- **4.2. `NtCreateDirectoryObject`** — handle falso.
- **4.3. `NtOpenDirectoryObject`** — handle falso (opaco).
- **4.4. `NtOpenKey` / `NtOpenKeyEx` / `NtOpenKeyTransacted` / `NtOpenKeyTransactedEx`** — todas devuelven un handle falso; el loader trata `STATUS_OBJECT_NAME_NOT_FOUND` como "usar default".
- **4.5. `NtQueryValueKey`** — devuelve `STATUS_OBJECT_NAME_NOT_FOUND` (ntdll asume valor por defecto y sigue).
- **4.6. `NtQueryOpenSubKeysEx`** — escribe `KEY_OPEN_SUBKEYS_INFORMATION` con `Count = 0`.
- **4.7. `NtOpenSymbolicLinkObject`** — handle falso + memoriza el target (necesario para `\KnownDlls\KnownDllPath` durante `LdrpInitializeProcess`).
- **4.8. `NtQuerySymbolicLinkObject`** — copia el target memorizado en el `UNICODE_STRING` del caller respetando `MaximumLength`.

### 5. `sync.rs`
- **5.1. `next_handle`** — contador global atómico que genera handles sintéticos consecutivos.
- **5.2. `NtCreateEvent`** — handle falso a `*EventHandle`.
- **5.3. `NtSetEvent`** — escribe el estado previo (0) si hay puntero y devuelve éxito.
- **5.4. `NtWaitForAlertByThreadId`** — retorna inmediatamente (no hay segundo hilo que despierte la espera).
- **5.5. `NtAlpcAcceptConnectPort`** — handle falso + éxito (acepta conexión).
- **5.6. `NtWaitForSingleObject`** — devuelve `STATUS_SUCCESS` (objeto señalado) al instante.
- **5.7. `NtOpenEvent`** — handle falso.
- **5.8. `NtCreateTimer2`** — handle falso.
- **5.9. `NtClose`** — log + éxito; no hay tabla real de handles que cerrar.

### 6. `system.rs`
- **6.1. `NtQuerySystemInformation`** — `match` por clase: `SystemBasic` (rellena PageSize, rangos VA, NumberOfProcessors=1), `Processor`, `Performance`, `TimeOfDay`, `ProcessInformation` (zero-fill mínimo), `KernelDebugger*` (DebuggerNotPresent=TRUE), `CodeIntegrity*`, `ExtendedHandleInformation` (0 handles), `SupportedProcessorArchitectures2`; clase desconocida → `STATUS_INVALID_INFO_CLASS`.
- **6.2. `NtManageHotPatch`** — `STATUS_NOT_SUPPORTED`.
- **6.3. `NtQueryDebugFilterState`** — devuelve 0 (sin debugger).
- **6.4. `NtTraceEvent`** — no-op (ETW no emulado).
- **6.5. `NtQueryInformationTransactionManager`** — KTM: clases 0 (Basic, 24 B) y 1 (Log, 16 B) → buffer a ceros + éxito; resto → `STATUS_INVALID_INFO_CLASS`.
- **6.6. `NtQueryIoCompletion`** — `STATUS_INVALID_HANDLE` (no se llevan colas reales).

### 7. `nls.rs`
- **7.1. `NtInitializeNlsFiles`** — `mmap`ea `locale.nls` desde `cfg.maps_folder` y devuelve su base + LCID (0x409 en-US).
- **7.2. `NtGetNlsSectionPtr`** — para `SectionType == 11` mapea el `C_<codepage>.NLS` correspondiente (1252, 437…).

### 8. `alpc.rs`
- **8.1. Helpers genéricos** — `alpc_handle_stub` (handle falso a `[RCX]`), `alpc_query_stub` (zero-fill del buffer), `alpc_void_stub` (sólo `STATUS_SUCCESS`).
- **8.2. `NtAlpcConnectPort`** — handle falso + zero-fill del `ConnectionMessage` para que `ReturnValue` quede en 0.
- **8.3. `NtAlpcCreatePort` / `…CreatePortSection` / `…CreateResourceReserve` / `…CreateSectionView` / `…CreateSecurityContext` / `NtAlpcConnectPortEx`** — todas usan `alpc_handle_stub`.
- **8.4. `NtAlpcQueryInformation` / `…QueryInformationMessage`** — `alpc_query_stub`: zero-fill + éxito.
- **8.5. `NtAlpcCancelMessage` / `…DeletePortSection` / `…DeleteResourceReserve` / `…DeleteSectionView` / `…DeleteSecurityContext` / `…DisconnectPort` / `…Impersonate*` / `…OpenSender*` / `…RevokeSecurityContext` / `…SetInformation`** — `alpc_void_stub` (sólo éxito).
- **8.6. `NtAlpcSendWaitReceivePort`** — escribe una cabecera `PORT_MESSAGE` mínima con `Type = LPC_REPLY` en el buffer de recepción para que `CsrClientConnectToServer` valide la respuesta.

---

## Windows x86 — `syscall/windows/syscall32/mod.rs`
- **9.1. `gateway`** — placeholder: cualquier nº de syscall se loguea como `(unimplemented)` sin alterar registros.

## Windows ntapi directo — `syscall/windows/ntapi/ntapi32.rs`
- **10.1. `gateway`** — sólo trata `0xdc` (`NtAlpcSendWaitReceivePort` → `rax=0`) y `0x10f` (`NtOpenFile` → `rax=0`); el resto provoca `unimplemented!()`.

---

## Linux x64 — `syscall/linux/syscall64/`

### 11. `mod.rs` + `misc.rs`
- **11.1. `dispatch_legacy_syscall64`** — gran `match` sobre `rax` que delega a los handlers de `fs.rs`, `memory.rs`, `net.rs`, `proc.rs` o emite trace simple cuando es un syscall sin efecto emulado (`ACCESS`, `FSTAT`, `STAT`, `READLINK`, `UNAME`, `ARCH_PRCTL`, `MMAP`, `MPROTECT`, `MUNMAP`, sockets `BIND`/`LISTEN`/`ACCEPT`/`CONNECT`/`SEND*`/`RECV*`/`SHUTDOWN`/`GETSOCKNAME`/`GETPEERNAME`/`GETSOCKOPT`/`SETSOCKOPT`/`SOCKETPAIR`).

### 12. `proc.rs`
- **12.1. `restart_syscall`** — sólo traza.
- **12.2. `exit`** — loguea el código en RDI y llama `emu.stop()`.
- **12.3. `fork`** — traza y abre la consola interactiva (`Console::spawn_console`).
- **12.4. `kill`** — traza PID y señal.
- **12.5. `dup` / `dup2`** — traza los fd implicados.

### 13. `fs.rs`
- **13.1. `read`** — lee de la "tabla de ficheros" emulada y vuelca contenido en el buffer del proceso.
- **13.2. `write`** — si fd ∈ {1,2} imprime el string del buffer como stdout/stderr; si no, traza.
- **13.3. `open` / `openat`** — registra el path, devuelve un fd falso.
- **13.4. `close`** — traza fd.
- **13.5. `execve`** — traza el path del binario (no se ejecuta nada).
- **13.6. `chdir`** — traza el path destino.
- **13.7. `chmod`** — traza path + modo.
- **13.8. `lseek`** — traza fd + offset.

### 14. `memory.rs`
- **14.1. `brk`** — devuelve la dirección actual del "break" emulado o la ajusta si se pide.
- **14.2. `nanosleep`** — no-op (no consume tiempo real).
- **14.3. `mremap`** — traza parámetros, no realoja realmente.

### 15. `net.rs`
- **15.1. `socket`** — crea un socket lógico vía `helper::socket_create()` y lo devuelve en RAX.

### 16. `signal.rs`
- **16.1. `dispatch`** — vacío; devuelve `false` para que el dispatcher siga buscando.

---

## Linux x86 — `syscall/linux/syscall32/`

### 17. `misc.rs`
- **17.1. `dispatch_legacy_syscall32`** — `match` sobre `eax` con ~150 entradas: los syscalls de fichero/proceso (`lseek`, `getpid`, `kill`, `dup`, `dup2`, `brk`, etc.) sólo se trazan; el nº **102** (`socketcall`) abre un sub-`match` sobre `ebx` que llama a `helper::socket_*` (SYS_SOCKET, SYS_BIND, SYS_CONNECT, SYS_LISTEN, SYS_ACCEPT, SYS_SEND*/RECV*, …) leyendo los argumentos desde la pila.

### 18. `fs.rs` / `memory.rs` / `proc.rs` / `net.rs` / `signal.rs`
- **18.1.** Esqueletos `dispatch` que devuelven `false` (toda la lógica útil vive en `misc.rs`).

---

## Linux aarch64 — `syscall/linux/syscall_aarch64.rs`

### 19. `gateway`
- **19.1. `exit` / `exit_group`** — `emu.stop()`.
- **19.2. `write`** — si fd 1/2 imprime el string como stdout/stderr; devuelve `count`.
- **19.3. `read`** — devuelve 0 (EOF, sin backing real).
- **19.4. `openat` / `close` / `lseek` / `fcntl` / `fstat` / `newfstatat` / `writev`** — traza los argumentos, sin efecto.
- **19.5. `brk`** — devuelve el break actual.
- **19.6. `mmap`** — reserva una página vía `emu.maps` con `Permission` derivada de `prot`.
- **19.7. `mprotect` / `munmap`** — actualizan o eliminan el mapa correspondiente.
- **19.8. `ioctl` / `futex` / `set_tid_address` / `rt_sigaction` / `rt_sigprocmask`** — no-op trazado, `x0 = 0`.
- **19.9. `clock_gettime`** — escribe `tv_sec=tick`, `tv_nsec=0` en el `timespec`.
- **19.10. `getpid` / `gettid`** — devuelve PID/TID falsos.
- **19.11. `uname`** — rellena el `utsname` con strings "Linux".
- **19.12. `getrandom`** — rellena el buffer con bytes seudoaleatorios (`rand`).

---

## macOS — `syscall/macos/`

### 20. `syscall_x86_64.rs` — `gateway`
Decodifica `rax` con la convención BSD (`0x2000000 | nr`); soporta:
- **20.1. `exit`** — `emu.stop()`.
- **20.2. `write`** — vuelca stdout/stderr si fd ∈ {1,2}.
- **20.3. `read` / `open` / `close` / `fork` / `mmap` / `munmap` / `mprotect` / `ioctl`** — trazan los argumentos y devuelven `u64::MAX` (`-1`) como retorno por defecto.
- **20.4. `issetugid`** — devuelve 0 (proceso no privilegiado).

### 21. `syscall_aarch64.rs` — `gateway`
Misma idea adaptada a ABI ARM64 (x0–x5 args, x16 = nº de syscall): `exit`, `read`, `write`, `open`, `close`, `fork`, `mmap`, `munmap`, `mprotect`, `ioctl`, `issetugid` — comportamiento equivalente al de x86_64.

---

## Notas
- Los `Nt*` se invocan tanto desde el modo `--ssdt` (la instrucción `syscall` real con `gateway` traduciendo el nº) como desde stubs de la WinAPI (`api/windows/winapi64/ntdll/*.rs`) que reusan la misma función.
- El patrón general en Windows: validar punteros (`emu.maps.is_mapped`), escribir el output (handles falsos vía `sync::next_handle()` o estructuras zero-filled mínimas), devolver `STATUS_SUCCESS` salvo cuando ntdll necesita un código específico para no spinlockear.
- Linux/macOS están mucho menos cubiertos: la mayoría son trazadores que nunca producen efecto en el estado del emulador.

---

# Backends de Filesystem / Registro

Coexisten dos infraestructuras para "ficheros" — una mínima que solo guarda strings en memoria, y otra con jail real sobre disco anfitrión.

## A. Backend "legacy" (en memoria) — `api/windows/helper.rs`

Usado por las syscalls Nt de `api/windows/winapi64/ntdll/file.rs` (`NtCreateFile`, `NtReadFile`, `NtClose`, `NtQueryInformationFile`, `NtSetInformationFile`).

```rust
struct Handler { id: u64, uri: String, data: Vec<u8> }
lazy_static! { static ref HANDLERS: Mutex<Vec<Handler>> = …; }
```

- `handler_create(uri)` → empuja un `Handler` con id incremental y devuelve el id como HANDLE.
- `handler_get_uri(h)` / `handler_find_by_uri` / `handler_put_bytes` / `handler_close` — operan sobre el `Vec` global.
- **No toca disco**: `NtCreateFile` solo memoriza la ruta extraída del `OBJECT_ATTRIBUTES`; `NtReadFile` tiene un único caso real (`\??\c:\cwd` lee del binario emulado, `emu.filename`), cualquier otra URI hace `panic!("TODO: read ...")`.
- `NtQueryInformationFile` / `NtSetInformationFile` devuelven `STATUS_SUCCESS` sin tocar nada.
- Sockets siguen el mismo patrón con `static ref SOCKETS: Mutex<Vec<u64>>` (fds reservados 0/1/2 para stdin/stdout/stderr).

## B. Backend jailed sobre disco — `emu/object_handle/`

Usado por las APIs win32 estilo `kernel32!CreateFileW` (ver `api/windows/winapi64/kernel32/create_file_w.rs`).

### B.1. Estructura general

| Fichero | LOC | Cometido |
|---|---|---|
| `windows_path.rs` | 373 | Tipo `WindowsPath { drive: Option<char>, folders: LinkedList<String> }`: parser de paths Win, soporta UNC (`\\?\…`), canoniza a minúsculas, expone `to_string`, `to_unc_path`, `to_portable_path` (forward slashes) y `to_device_path` (`\Device\HarddiskVolumeN\…`). Tests propios en el módulo. |
| `file_handle.rs` | 687 | `FileSystem`, `FileSystemBuilder`, `FileHandle`, `init_file_system`, `FILE_SYSTEM: OnceLock<FileSystem>`. |
| `mod.rs` | 113 | `HandleManagement` con `slab::Slab<HandleType>` (variantes `FileHandle`, `MappingHandle`); la *key* del slab se devuelve al guest como HANDLE (`u32`). |
| `mapping_handle.rs` | 17 | Esqueleto para `CreateFileMapping`. |
| `registry_handle.rs` | 273 | Tipos para emular el registro — **aún no enchufado** a `syscall/windows/syscall64/registry.rs`. |
| `hive_parser.rs` | 907 | Parser binario de hives del registro Windows (NT format) — código presente pero **sin integración** con las syscalls actuales. |

### B.2. El "jail" (chroot)

Inicialización (`crates/mwemu/src/main.rs:564`):
```rust
init_file_system(None as Option<PathBuf>);   // root por defecto
```
- Sin argumento → root = `<dir_del_ejecutable>/file_root/`. Se crea con `fs::create_dir_all`.
- Con argumento → se acepta tanto absoluto como relativo al ejecutable.
- El root queda guardado en `FILE_SYSTEM: OnceLock<FileSystem>` (singleton de proceso).

```rust
pub struct FileSystem {
    root: PathBuf,
    mappings: AHashMap<WindowsPath, PathBuf>,
}
```

Traducción guest → host (`FileSystem::translate`, file_handle.rs:394):

1. Solo paths **absolutos** (con drive); de lo contrario error.
2. Si la `WindowsPath` está en `mappings`, devuelve directamente el `PathBuf` mapeado (overrides explícitos tipo `D:\Games → /mnt/d/Games`).
3. Si no, construye `<root>/<drive>/<folders>` vía `win_path.to_portable_path()` → p.ej. `C:\Windows\System32\foo` → `<root>/c/Windows/System32/foo`.
4. **`soft_canonicalize`** resuelve `..` y symlinks.
5. **`is_subpath(<root>/<drive>, path)`** confirma contención. Si la ruta canónica se sale del root del drive devuelve el root como *fail-safe* (no error).

Es un chroot real: `..\..\..\..\..\etc\passwd` queda dentro del jail.

### B.3. El handle real

`FileHandle::new(name, access_mode, creation_disposition, flags_and_attributes, sharing_mode)` — `file_handle.rs:167`:

- Parsea `name` → `WindowsPath`.
- `FILE_SYSTEM.get().translate(...)` → `PathBuf` anfitrión.
- `fs::metadata` para distinguir fichero/directorio (los directorios devuelven error: aún no soportados).
- `match creation_disposition`:
  - `1 CREATE_NEW` → `File::create_new`
  - `2 CREATE_ALWAYS` → `File::create`
  - `3 OPEN_EXISTING` → `File::open`
  - `4 OPEN_ALWAYS` → `File::options().read().write().create().open()`
  - `5 TRUNCATE_EXISTING` → `read().write().truncate().open()`
- Devuelve un `FileHandle` con:
  ```rust
  { name, path, is_dir, file: Option<File>,
    access_mode, creation_disposition, flags_and_attributes, sharing_mode,
    file_position: u64, is_valid: bool, is_eof: bool, file_size: u64 }
  ```
- Métodos: `read` / `write` / `seek` / `set_position` / `close` — proxy directo al `std::fs::File` real, manteniendo `file_position` y `is_eof` actualizados.
- El handle se inserta en `emu.handle_management.insert_file_handle(fh)` → key `u32` que va a `rax` como HANDLE.

### B.4. Flujo en `CreateFileW`

`api/windows/winapi64/kernel32/create_file_w.rs`:

1. Lee args (`lp_file_name_wide`, `desired_access`, `share_mode`, `creation_disposition`, …).
2. `emu.maps.read_wide_string(lp_file_name_wide)` → `String` Windows.
3. `FILE_SYSTEM.get().local_to_windows_path(&name)` — normaliza vía el FileSystem global.
4. `FileHandle::new(...)` → abre el fichero en el host (dentro del jail).
5. `emu.handle_management.insert_file_handle(file_handle)` → key como HANDLE.
6. `rax = key` en éxito, o `INVALID_HANDLE_VALUE = !0` en fallo (log con `error!`).

## C. Resumen del estado actual

| Capa | Toca disco | Jailed | Estado |
|---|---|---|---|
| `syscall/windows/syscall64/registry.rs` | No | n/a | Stubs (handles falsos + `STATUS_OBJECT_NAME_NOT_FOUND`). |
| `api/.../ntdll/file.rs` (`NtCreateFile`…) | No (excepto `\??\c:\cwd`) | n/a | Handlers en memoria (`helper.rs`). |
| `api/.../kernel32/CreateFileW` y derivados | Sí | Sí (`file_root/`) | Funcional, con `WindowsPath` + `is_subpath`. |
| `emu/object_handle/registry_handle.rs` + `hive_parser.rs` | (en construcción) | — | Código presente pero no conectado a las `Nt*Key`. |

En la práctica: si quisieras que las syscalls Nt de fichero usaran el jail real, habría que reescribir `winapi64/ntdll/file.rs` para que llamen a `FileHandle::new` y `emu.handle_management` en lugar de a `helper::handler_create`. Para el registro habría que enchufar `hive_parser` + `registry_handle` desde `syscall/windows/syscall64/registry.rs` (hoy todos esos `Nt*Key` ignoran cualquier contenido).

---

# Vulnerabilidades / superficies de ataque conocidas

Modelo de amenaza: el emulador corre malware desconocido. El binario emulado controla todos los argumentos (registros, contenido de memoria emulada, RSP) de syscalls y APIs. "Vuln" aquí significa que el guest puede afectar al **proceso emulador en el host** más allá de "ejecutarse dentro del sandbox de emulación".

## Críticas (rompen jail / fail-open)

### V1. `CreateFileW` invoca la función incorrecta
`api/windows/winapi64/kernel32/create_file_w.rs:42-47`
```rust
let emu_path = temp_emu.and_then(|fs| fs.local_to_windows_path(&name_utf8).ok());
let emu_path_str = emu_path.unwrap().to_string();
```
`local_to_windows_path` (file_handle.rs:447) está pensado para el reverso **host → guest** (recibe un path local del anfitrión y devuelve `WindowsPath`). Aquí se le pasa la cadena cruda del guest ("C:\\Windows\\System32\\foo.dll"). En Linux, `soft_canonicalize` la procesa como path local y casi siempre devuelve `Err` → `.ok()` = `None` → `.unwrap()` → **panic del emulador**.

Patch: usar `WindowsPath::from_string(&name_utf8)` + `fs.translate(&wp)`.

### V2. `FileHandle::new` hace `.unwrap()` sobre `translate(...).ok()`
`file_handle.rs:177`
```rust
let resolved_path = FILE_SYSTEM.get()
    .and_then(|fs| fs.translate(&windows_path).ok())
    .unwrap();
```
`translate` devuelve `Err` cuando: path no absoluto, `soft_canonicalize` falla, etc. `FILE_SYSTEM.get()` devuelve `None` si nadie llamó `init_file_system`. Cualquiera de los dos → panic del emulador. DoS por entrada controlable.

### V3. `FileSystem::translate` fail-open al salirse del jail
`file_handle.rs:418-421`
```rust
if Self::is_subpath(&root_path, &path) { Ok(path) } else { Ok(root_path) }
```
Cuando el path canónico se escapa del jail, no se devuelve `Err`: se devuelve el directorio `<root>/<drive>`. Luego `fs::metadata` lo ve como directorio, `FileHandle::new` devuelve error y, encadenado con V2, **panic**. Además semánticamente es "fail-open con downgrade", no contención dura.

### V4. Modo "sin jail" silencioso en Windows
`file_handle.rs:405-408`
```rust
#[cfg(target_os = "windows")]
if self.root.as_os_str().is_empty() {
    return Ok(PathBuf::from(win_path.to_string()));
}
```
Si el root queda vacío (config explícita, builder mal usado, dump cargado), **el jail desaparece** y el malware accede al filesystem completo del host. Hoy el default `init_file_system(None)` impide caer ahí, pero es una rama sin assertion.

### V5. Escritura sin restricción en el jail
`FileHandle::new` honra `CREATE_NEW` / `CREATE_ALWAYS` / `OPEN_ALWAYS` / `TRUNCATE_EXISTING` → el guest crea, sobrescribe y trunca ficheros reales bajo `<root>`. No hay:
- límite de tamaño por fichero / agregado,
- límite de número de handles,
- filtro por extensión,
- modo read-only para análisis estático.

DoS por llenar disco; persistencia / plantado de artefactos en `file_root/`.

### V6. TOCTOU symlink entre `soft_canonicalize` y `File::open`
`translate` canoniza → `is_subpath` valida → devuelve `PathBuf`. Después `FileHandle::new` hace `fs::metadata` + `File::open` con el mismo string. Si entre medias un componente se reemplaza por un symlink fuera del jail (otro proceso del usuario, FS compartido, etc.), `File::open` lo sigue. Mitigación robusta: `openat2(... RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS)` en Linux o equivalente.

## Medias (DoS, paths exóticos)

### V7. `WindowsPath::from_string` no filtra nada
`windows_path.rs:48-94`: acepta `..`, `.`, `<`, `>`, `|`, `\0`, prefijos `\\.\PIPE\…`, `\\?\GLOBALROOT\…`, NT paths `\??\C:\…` (queda mal parseado: folders `["??", "C:", …]`, sin drive → no es absolute → `translate` Err → panic vía V2). El `..` no es traversal (lo come `soft_canonicalize` + `is_subpath`), pero los prefijos especiales sí provocan DoS encadenado.

### V8. `NtReadFile` panic por defecto
`api/windows/winapi64/ntdll/file.rs:429`
```rust
} else { panic!("TODO: read {}", filename); }
```
Cualquier `NtReadFile` cuya URI no sea exactamente `\??\c:\cwd` mata el emulador.

### V9. `.expect()` masivo sobre args de syscalls Nt
`winapi64/ntdll/file.rs` ~30 ocurrencias de `emu.maps.read_qword(rsp+0xN).expect("...")` sobre stack args controlables por el guest (RSP lo pone el guest). Cualquier RSP que apunte a memoria no mapeada → panic.

### V10. `helper.rs`: handlers globales sin límite
`HANDLERS: Mutex<Vec<Handler>>` (helper.rs:58) sin tope. `Handler` lleva un `Vec<u8>` por entrada. Loop de `NtCreateFile` → memoria del proceso emulador explota. `handler_close` además hace `Vec::remove(idx)` (O(n)).

### V11. `NtCreateFile` no valida `out_hndl_ptr` antes de escribir
`winapi64/ntdll/file.rs:300-303`: `emu.maps.write_qword(out_hndl_ptr, ...)` sin `is_mapped`. El resto del proyecto valida primero; aquí no.

### V12. `read_wide_string_n` con `length` controlado
`winapi64/ntdll/file.rs:276-277`: `length` viene del `UNICODE_STRING` del guest. Hasta 64 KB leídos por llamada. No grave aislado, contribuye a DoS por amplificación.

## Bajas / cosméticas

### V13. `RtlDosPathNameToNtPathName_U` allocea y nunca libera
`winapi64/ntdll/file.rs:62-86`: cada invocación `emu.maps.alloc()` + `create_map("nt_path_string_*")`. Loop guest → agota el espacio del emulador (no del host).

### V14. `WindowsPath::from_string` siempre lowercase
`windows_path.rs:75`: `to_ascii_lowercase()` aplicado a folders. En Linux los lookups bajo `<root>/c/windows/system32/…` requieren que el `file_root` esté preparado en minúsculas. Bug funcional, no de seguridad.

### V15. Posible bug en `FileSystem::relative` / `is_subpath`
No auditado a fondo: si `relative()` se implementa por `&str`-strip en vez de component-aware, podría confundir `<root>` con `<root>foo`. Verificar.

## Recomendaciones priorizadas

1. **V1 + V2**: eliminar `.unwrap()` en `CreateFileW` y `FileHandle::new`; devolver `INVALID_HANDLE_VALUE` con log en lugar de panic.
2. **V3**: `translate` debe devolver `Err` al salirse del jail.
3. **V4**: rechazar root vacío en `init_file_system` (`Err` explícito).
4. **V8 + V9**: sustituir `panic!` y `.expect()` por `STATUS_*` y `INVALID_HANDLE_VALUE`.
5. **V10**: tope al `HANDLERS` (p.ej. 4096) y devolver `STATUS_INSUFFICIENT_RESOURCES`.
6. **V5**: añadir flag `--ro-fs` que mapea `CREATE_*`/`OPEN_ALWAYS`/`TRUNCATE_EXISTING` a `STATUS_ACCESS_DENIED`.
7. **V6**: en Linux usar `openat2(RESOLVE_BENEATH|RESOLVE_NO_SYMLINKS)`; en Windows usar `CreateFileW` con `FILE_FLAG_OPEN_REPARSE_POINT` y validación de prefijo NT.
8. **V7**: parser de `WindowsPath` que reconozca o rechace prefijos `\??\`, `\\.\`, `\\?\GLOBALROOT\`, `\\?\UNC\`.
