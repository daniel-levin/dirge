//! # Project Deployment Tool
//!
//! This example demonstrates the true value of dirge's type-safe path types
//! by implementing a project deployment tool where different path constraints
//! prevent common deployment bugs.
//!
//! ## Key Demonstrations:
//!
//! 1. **AbsPath**: Server configurations require absolute paths to prevent
//!    deployment to wrong directories when working directory changes
//!
//! 2. **RelPath**: Source file patterns are relative to project root,
//!    preventing accidental system-wide operations
//!
//! 3. **NormPath**: User-provided paths are normalized to prevent directory
//!    traversal attacks and resolve symbolic links consistently
//!
//! ## Real-world Benefits:
//!
//! - Compile-time prevention of path-related security vulnerabilities
//! - Clear API contracts about path requirements
//! - Automatic path normalization prevents injection attacks
//! - Type system guides developers to handle paths correctly

use dirge::{
    AbsPath, AbsPathBuf, NormPath, NormPathBuf, RelPathBuf, ToAbsPathBuf, ToNormPathBuf,
    ToRelPathBuf,
};
use std::{collections::HashMap, io};

/// Deployment configuration that enforces path type safety
#[derive(Debug)]
pub struct DeploymentConfig {
    /// Server deployment target - MUST be absolute to prevent accidents
    pub target_dir: AbsPathBuf,
    /// Project source patterns - MUST be relative to project root
    pub source_patterns: Vec<RelPathBuf>,
    /// Excluded paths - normalized to prevent traversal attacks
    pub excluded_paths: Vec<NormPathBuf>,
}

/// Server configuration that requires absolute paths for safety
#[derive(Debug)]
pub struct ServerConfig {
    /// Root directory - absolute path prevents working directory issues
    pub root_dir: AbsPathBuf,
    /// Log directory - absolute path ensures logs go to correct location
    pub log_dir: AbsPathBuf,
    /// Config file - absolute path prevents config file confusion
    pub config_file: AbsPathBuf,
}

/// File deployment manifest with type-safe path handling
#[derive(Debug)]
pub struct DeploymentManifest {
    files: HashMap<RelPathBuf, AbsPathBuf>,
}

impl DeploymentConfig {
    /// Create a new deployment configuration
    ///
    /// This function signature makes it impossible to accidentally pass
    /// relative paths where absolute paths are required, or vice versa.
    pub fn new(
        target_dir: AbsPathBuf,
        source_patterns: Vec<RelPathBuf>,
        excluded_paths: Vec<NormPathBuf>,
    ) -> Self {
        Self {
            target_dir,
            source_patterns,
            excluded_paths,
        }
    }

    /// Validate that all source patterns are truly relative
    ///
    /// This demonstrates how the type system prevents a common bug:
    /// accidentally using absolute paths in source patterns.
    pub fn validate_source_patterns(&self) -> Result<(), String> {
        for pattern in &self.source_patterns {
            if pattern.is_absolute() {
                return Err(format!(
                    "Source pattern {:?} cannot be absolute - this would cause system-wide operations",
                    pattern
                ));
            }
        }
        Ok(())
    }

    /// Check if a path should be excluded
    ///
    /// By using NormPath, we automatically prevent directory traversal
    /// attacks like "../../../etc/passwd" in exclude patterns.
    pub fn is_excluded(&self, path: &NormPath) -> bool {
        self.excluded_paths
            .iter()
            .any(|excluded| path.starts_with(excluded))
    }
}

impl ServerConfig {
    /// Create server configuration with absolute path validation
    ///
    /// The type system guarantees that all paths are absolute,
    /// preventing configuration errors that could cause the server
    /// to fail when the working directory changes.
    pub fn new(
        root_dir: AbsPathBuf,
        log_dir: AbsPathBuf,
        config_file: AbsPathBuf,
    ) -> io::Result<Self> {
        // In a real deployment, you'd validate that paths exist
        // For this demo, we'll just show the type safety benefits
        println!("   📁 Root directory: {:?}", root_dir);
        println!("   📁 Log directory: {:?}", log_dir);
        println!("   📄 Config file: {:?}", config_file);

        Ok(Self {
            root_dir,
            log_dir,
            config_file,
        })
    }

    /// Get the log file path for a specific component
    ///
    /// By using AbsPath, we guarantee that log files will always
    /// be written to the correct location regardless of working directory.
    pub fn log_file_for(&self, component: &str) -> AbsPathBuf {
        let mut log_file = self.log_dir.to_owned();
        log_file.push(format!("{}.log", component));
        log_file
    }
}

impl Default for DeploymentManifest {
    fn default() -> Self {
        Self::new()
    }
}

impl DeploymentManifest {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Add a file mapping from relative source to absolute target
    ///
    /// This API makes it impossible to accidentally swap source and target paths,
    /// a common bug in deployment scripts.
    pub fn add_file(&mut self, source: RelPathBuf, target: AbsPathBuf) {
        self.files.insert(source, target);
    }

    /// Deploy all files in the manifest
    ///
    /// The type system ensures we can't accidentally deploy to relative paths,
    /// which could overwrite files in unexpected locations.
    pub fn deploy(&self, base_dir: &AbsPath) -> io::Result<()> {
        for (source, target) in &self.files {
            let source_path = base_dir.join(source);
            println!("Deploying {:?} -> {:?}", source_path, target);

            // In a real implementation, you would copy the file here
            // std::fs::copy(&source_path, target)?;
        }
        Ok(())
    }
}

/// Secure path processor that demonstrates normalization benefits
pub struct SecurePathProcessor;

impl SecurePathProcessor {
    /// Process user-provided paths safely
    ///
    /// By normalizing user input, we prevent directory traversal attacks
    /// and ensure consistent path handling.
    pub fn process_user_path(user_input: &str) -> io::Result<NormPathBuf> {
        // This would be dangerous with raw strings:
        // User could input: "../../etc/passwd"
        // But normalization makes it safe:
        let normalized = user_input.to_norm_path_buf()?;

        println!("User input: {:?}", user_input);
        println!("Normalized: {:?}", normalized);

        Ok(normalized)
    }

    /// Validate that a normalized path doesn't escape a sandbox
    pub fn validate_sandbox_escape(path: &NormPath, sandbox: &AbsPath) -> Result<(), String> {
        // Convert to absolute path for proper validation
        let abs_path = sandbox.join(path);

        if !abs_path.starts_with(sandbox) {
            return Err(format!(
                "Path {:?} would escape sandbox {:?}",
                path, sandbox
            ));
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    println!("🚀 Deployment Tool - Demonstrating Type-Safe Paths\n");

    // 1. Server configuration with absolute path safety
    println!("1. Creating server configuration...");
    let server_config = ServerConfig::new(
        "/opt/myapp".to_abs_path_buf()?,
        "/var/log/myapp".to_abs_path_buf()?,
        "/etc/myapp/config.toml".to_abs_path_buf()?,
    )?;

    println!("   ✅ Server configuration created successfully!");
    println!(
        "   📝 Deployment log: {:?}",
        server_config.log_file_for("deployment")
    );

    // 2. Deployment configuration with mixed path types
    println!("\n2. Creating deployment configuration...");
    let deployment_config = DeploymentConfig::new(
        "/opt/myapp/releases/v1.0.0".to_abs_path_buf()?,
        vec![
            "src/**/*.rs".to_rel_path_buf()?,
            "assets/**/*".to_rel_path_buf()?,
            "Cargo.toml".to_rel_path_buf()?,
        ],
        vec![
            "target/debug".to_norm_path_buf()?,
            ".git".to_norm_path_buf()?,
            "temp/../cache".to_norm_path_buf()?, // Gets normalized to "cache"
        ],
    );

    println!("   Target: {:?}", deployment_config.target_dir);
    println!("   Sources: {:?}", deployment_config.source_patterns);
    println!("   Excluded: {:?}", deployment_config.excluded_paths);

    // 3. Demonstrate path normalization security
    println!("\n3. Processing user-provided paths (normalization demo)...");
    let dangerous_inputs = [
        "normal/path.txt",
        "./current/dir/file.txt",
        "path/../other/file.txt",
        "../../etc/passwd", // This gets normalized safely
        "path/./to/../file.txt",
    ];

    for input in dangerous_inputs {
        match SecurePathProcessor::process_user_path(input) {
            Ok(normalized) => {
                // Validate against sandbox escape
                let sandbox = "/opt/myapp/data".to_abs_path_buf()?;
                match SecurePathProcessor::validate_sandbox_escape(&normalized, &sandbox) {
                    Ok(()) => println!("   ✅ Safe: {} -> {:?}", input, normalized),
                    Err(e) => println!("   ⚠️  Blocked: {} -> {}", input, e),
                }
            }
            Err(e) => println!("   ❌ Invalid: {} -> {}", input, e),
        }
    }

    // 4. Create deployment manifest
    println!("\n4. Building deployment manifest...");
    let mut manifest = DeploymentManifest::new();

    // Type system prevents this common mistake:
    // manifest.add_file(absolute_path, relative_path); // Won't compile!

    manifest.add_file(
        "src/main.rs".to_rel_path_buf()?,
        "/opt/myapp/releases/v1.0.0/src/main.rs".to_abs_path_buf()?,
    );
    manifest.add_file(
        "assets/logo.png".to_rel_path_buf()?,
        "/opt/myapp/releases/v1.0.0/assets/logo.png".to_abs_path_buf()?,
    );

    // 5. Deploy files (simulation)
    println!("\n5. Deploying files...");
    let project_root = "/home/user/myproject".to_abs_path_buf()?;
    manifest.deploy(&project_root)?;

    println!("\n✅ Deployment completed successfully!");
    println!("\n💡 Notice how the type system prevented common bugs:");
    println!("   - Can't use relative paths for server config");
    println!("   - Can't use absolute paths for source patterns");
    println!("   - User input is automatically normalized");
    println!("   - Can't accidentally swap source/target in manifest");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_config_validation() {
        let config = DeploymentConfig::new(
            "/opt/app".to_abs_path_buf().unwrap(),
            vec!["src/**/*.rs".to_rel_path_buf().unwrap()],
            vec!["target".to_norm_path_buf().unwrap()],
        );

        assert!(config.validate_source_patterns().is_ok());
    }

    #[test]
    fn test_path_normalization() {
        let normalized = "path/./to/../file.txt".to_norm_path_buf().unwrap();
        assert_eq!(normalized.to_string_lossy(), "path/file.txt");
    }

    #[test]
    fn test_exclusion_check() {
        let config = DeploymentConfig::new(
            "/opt/app".to_abs_path_buf().unwrap(),
            vec![],
            vec!["target".to_norm_path_buf().unwrap()],
        );

        let test_path = "target/debug/myapp".to_norm_path_buf().unwrap();
        assert!(config.is_excluded(&test_path));
    }
}
