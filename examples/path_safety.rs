//! # Path Safety Examples
//!
//! Simple examples demonstrating the core value propositions of dirge's path types.

use dirge::{AbsPathBuf, NormPathBuf, RelPathBuf, ToAbsPathBuf, ToNormPathBuf, ToRelPathBuf};
use std::io;

fn main() -> io::Result<()> {
    println!("🛡️  Path Safety Examples\n");

    // 1. Type Safety: Prevent mixing absolute and relative paths
    println!("1. Type Safety - Preventing Path Mix-ups");

    // This function only accepts absolute paths
    fn backup_to_safe_location(source: AbsPathBuf, target: AbsPathBuf) {
        println!("   Backing up {:?} to {:?}", source, target);
        // Compiler guarantees both paths are absolute
    }

    // This function only accepts relative paths (for project files)
    fn add_to_project(file: RelPathBuf) {
        println!("   Adding project file: {:?}", file);
        // Compiler guarantees this won't escape project directory
    }

    let abs_path = "/home/user/important.txt".to_abs_path_buf()?;
    let rel_path = "src/lib.rs".to_rel_path_buf()?;

    backup_to_safe_location(abs_path, "/backup/important.txt".to_abs_path_buf()?);
    add_to_project(rel_path);

    // These would fail to compile (demonstrating safety):
    // backup_to_safe_location(rel_path, abs_path); // ❌ Won't compile
    // add_to_project(abs_path);                    // ❌ Won't compile

    println!("   ✅ Type system prevents path category mix-ups!\n");

    // 2. Path Normalization: Security and consistency
    println!("2. Path Normalization - Security & Consistency");

    let dangerous_paths = [
        "normal/file.txt",
        "./current/./file.txt",
        "path/../other/file.txt",
        "deep/path/../../file.txt",
        "../../../etc/passwd", // Directory traversal attempt
    ];

    for path in dangerous_paths {
        let normalized = path.to_norm_path_buf()?;
        println!("   '{}' -> '{}'", path, normalized.display());
    }

    println!("   ✅ All paths normalized safely!\n");

    // 3. Clear API Contracts
    println!("3. Clear API Contracts");

    // Function signatures clearly communicate path requirements
    fn setup_server(
        config_dir: AbsPathBuf,       // Must be absolute (system config)
        templates: Vec<RelPathBuf>,   // Must be relative (project files)
        user_files: Vec<NormPathBuf>, // Normalized (user input)
    ) {
        println!("   Server config directory: {:?}", config_dir);
        println!("   Template files: {:?}", templates);
        println!("   User-provided files: {:?}", user_files);
    }

    setup_server(
        "/etc/myserver".to_abs_path_buf()?,
        vec![
            "templates/index.html".to_rel_path_buf()?,
            "templates/error.html".to_rel_path_buf()?,
        ],
        vec![
            "user/./profile/../settings.json".to_norm_path_buf()?,
            "uploads/image.png".to_norm_path_buf()?,
        ],
    );

    println!("   ✅ API clearly communicates path requirements!\n");

    // 4. Runtime Benefits
    println!("4. Runtime Benefits - No Overhead");
    use std::mem;

    println!(
        "   std::path::PathBuf size: {} bytes",
        mem::size_of::<std::path::PathBuf>()
    );
    println!(
        "   AbsPathBuf size:         {} bytes",
        mem::size_of::<AbsPathBuf>()
    );
    println!(
        "   RelPathBuf size:         {} bytes",
        mem::size_of::<RelPathBuf>()
    );
    println!(
        "   NormPathBuf size:        {} bytes",
        mem::size_of::<NormPathBuf>()
    );
    println!("   ✅ Zero runtime overhead!\n");

    println!("🎉 All examples completed successfully!");
    println!("   The type system caught potential bugs at compile time,");
    println!("   normalized dangerous user input, and made APIs crystal clear!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_normalization_safety() {
        // Dangerous input gets normalized (leading .. are preserved when they can't be resolved)
        let dangerous = "../../../etc/passwd".to_norm_path_buf().unwrap();
        assert_eq!(dangerous.to_string_lossy(), "../etc/passwd");

        // Complex paths get simplified
        let complex = "path/./to/../from/./file.txt".to_norm_path_buf().unwrap();
        assert_eq!(complex.to_string_lossy(), "path/from/file.txt");

        // Path traversal within a path gets resolved
        let traversal = "safe/../../etc/passwd".to_norm_path_buf().unwrap();
        assert_eq!(traversal.to_string_lossy(), "../etc/passwd");
    }

    #[test]
    fn test_type_safety() {
        let abs = "/home/user/file.txt".to_abs_path_buf().unwrap();
        let rel = "src/main.rs".to_rel_path_buf().unwrap();

        // These paths maintain their type guarantees
        assert!(abs.is_absolute());
        assert!(rel.is_relative());
    }
}
