use libloading::{Library, Symbol};
use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

type ConvertXmlToDaeFn = unsafe extern "C" fn(
    pszGameBaseFolderPath: *const c_char,
    pszXmlFilePath: *const c_char,
    pszDaeFilePath: *const c_char,
    pszError: *mut c_char,
    iMaxErrorSize: i32,
) -> bool;

type ConvertDaeToXmlFn = unsafe extern "C" fn(
    pszGameBaseFolderPath: *const c_char,
    pszDaeFilePath: *const c_char,
    pszXmlFilePath: *const c_char,
    pszError: *mut c_char,
    iMaxErrorSize: i32,
) -> bool;

fn main() {
    // Collect arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!(
            "Usage: {} <action> <game_base_folder> <input_file_path>",
            args[0]
        );
        eprintln!("Actions:");
        eprintln!("  importxmf: Convert XML to DAE");
        eprintln!("  exportxmf: Convert DAE to XML");
        std::process::exit(1);
    }

    let action = &args[1];
    let game_base_folder = &args[2];
    let input_file_path = &args[3];

    // Determine output file path by replacing extension
    let output_extension = match action.as_str() {
        "importxmf" => "dae",
        "exportxmf" => "xml",
        _ => {
            eprintln!("Unknown action: {}", action);
            std::process::exit(1);
        }
    };

    let output_file_path = replace_extension(input_file_path, output_extension);
    let game_base_folder_cstr = CString::new(game_base_folder.as_str()).expect("Invalid CString");
    let input_file_path_cstr = CString::new(input_file_path.as_str()).expect("Invalid CString");
    let output_file_path_cstr = CString::new(output_file_path.as_str()).expect("Invalid CString");
    unsafe {
        let lib_path = curdir()
            .map(|dir| dir.join("XRConverters.dll")) 
            .expect("Failed to determine XRConverters.dll directory");

        let lib = Library::new(lib_path).expect("Could not load the DLL");

        match action.as_str() {
            "importxmf" => {
                let convert_xml_to_dae: Symbol<ConvertXmlToDaeFn> =
                    lib.get(b"ConvertXmlToDae").expect("Could not find ConvertXmlToDae");

                let mut error_buffer = vec![0i8; 256];
                let result = convert_xml_to_dae(
                    game_base_folder_cstr.as_ptr(),
                    input_file_path_cstr.as_ptr(),
                    output_file_path_cstr.as_ptr(),
                    error_buffer.as_mut_ptr(),
                    error_buffer.len() as i32,
                );

                if result {
                    println!("XML to DAE conversion succeeded: {}", output_file_path);
                } else {
                    let error_message = CStr::from_ptr(error_buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned();
                    println!("XML to DAE conversion failed: {}", error_message);
                }
            }
            "exportxmf" => {
                let convert_dae_to_xml: Symbol<ConvertDaeToXmlFn> =
                    lib.get(b"ConvertDaeToXml").expect("Could not find ConvertDaeToXml");

                let mut error_buffer = vec![0i8; 256];
                let result = convert_dae_to_xml(
                    game_base_folder_cstr.as_ptr(),
                    input_file_path_cstr.as_ptr(),
                    output_file_path_cstr.as_ptr(),
                    error_buffer.as_mut_ptr(),
                    error_buffer.len() as i32,
                );

                if result {
                    println!("DAE to XML conversion succeeded: {}", output_file_path);
                } else {
                    let error_message = CStr::from_ptr(error_buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned();
                    eprintln!("DAE to XML conversion failed: {}", error_message);
                }
            }
            _ => {
                eprintln!("Unknown action: {}", action);
                std::process::exit(1);
            }
        }
    }
}

/// Replace the file extension of a given path
fn replace_extension(input_path: &str, new_extension: &str) -> String {
    let path = Path::new(input_path);
    let mut new_path = PathBuf::from(path);
    new_path.set_extension(new_extension);
    new_path
        .to_str()
        .expect("Failed to construct new path")
        .to_string()
}

/// Determine the directory of the current executable
fn curdir() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(PathBuf::from))
}
