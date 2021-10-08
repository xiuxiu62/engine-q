use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EvaluationContext};
use nu_protocol::{ShellError, Signature, SyntaxShape, Value};

pub struct Rm;

impl Command for Rm {
    fn name(&self) -> &str {
        "rm"
    }

    fn usage(&self) -> &str {
        "Remove file(s)."
    }

    fn signature(&self) -> Signature {
        Signature::build("rm")
            .switch(
                "trash",
                "use the platform's recycle bin instead of permanently deleting",
                Some('t'),
            )
            .switch(
                "permanent",
                "don't use recycle bin, delete permanently",
                Some('p'),
            )
            .switch("recursive", "delete subdirectories recursively", Some('r'))
            .switch("force", "suppress error when no file", Some('f'))
            .rest(
                "rest",
                SyntaxShape::GlobPattern,
                "the file path(s) to remove",
            )
    }

    fn run(
        &self,
        context: &EvaluationContext,
        call: &Call,
        _input: Value,
    ) -> Result<Value, ShellError> {
        unimplemented!();
    }
}

// fn test_run() {
// 	        fn rm(args: CommandArgs) -> Result<ActionStream, ShellError> {
//             let name = args.call_info.name_tag.clone();
//             let shell_manager = args.shell_manager();

//             let mut rm_args = RemoveArgs {
//                 rest: args.rest(0)?,
//                 recursive: args.has_flag("recursive"),
//                 trash: args.has_flag("trash"),
//                 permanent: args.has_flag("permanent"),
//                 force: args.has_flag("force"),
//             };

//             if rm_args.trash && rm_args.permanent {
//                 return Ok(ActionStream::one(Err(ShellError::labeled_error(
//                     "only one of --permanent and --trash can be used",
//                     "conflicting flags",
//                     name,
//                 ))));
//             }

//             if rm_args.rest.is_empty() {
//                 let mut input_peek = args.input.peekable();
//                 while let Some(v) = &input_peek.next() {
//                     if let UntaggedValue::Primitive(Primitive::FilePath(path)) = &v.value {
//                         rm_args.rest.push(Tagged {
//                             item: path.to_path_buf(),
//                             tag: args.call_info.name_tag.clone(),
//                         })
//                     };
//                 }
//             }

//             shell_manager.rm(rm_args, name)
// 			}

// 	fn test_run_rm(
//         &self,
//         RemoveArgs {
//             rest: targets,
//             recursive,
//             trash: _trash,
//             permanent: _permanent,
//             force: _force,
//         }: RemoveArgs,
//         name: Tag,
//         path: &str,
//     ) -> Result<ActionStream, ShellError> {
//         let rm_always_trash = nu_data::config::config(Tag::unknown())?
//             .get("rm_always_trash")
//             .map(|val| val.is_true())
//             .unwrap_or(false);

//         #[cfg(not(feature = "trash-support"))]
//         {
//             if rm_always_trash {
//                 return Err(ShellError::untagged_runtime_error(
//                     "Cannot execute `rm`; the current configuration specifies \
//                     `rm_always_trash = true`, but the current nu executable was not \
//                     built with feature `trash_support`.",
//                 ));
//             } else if _trash {
//                 return Err(ShellError::labeled_error(
//                     "Cannot execute `rm` with option `--trash`; feature `trash-support` not enabled",
//                     "this option is only available if nu is built with the `trash-support` feature",
//                     name
//                 ));
//             }
//         }

//         let name_tag = name;

//         if targets.is_empty() {
//             return Err(ShellError::labeled_error(
//                 "rm requires target paths",
//                 "needs parameter",
//                 name_tag,
//             ));
//         }

//         let path = Path::new(path);
//         let mut all_targets: HashMap<PathBuf, Tag> = HashMap::new();
//         for target in targets {
//             let all_dots = target
//                 .item
//                 .to_str()
//                 .map_or(false, |v| v.chars().all(|c| c == '.'));

//             if all_dots {
//                 return Err(ShellError::labeled_error(
//                     "Cannot remove any parent directory",
//                     "cannot remove any parent directory",
//                     target.tag,
//                 ));
//             }

//             let path = path.join(&target.item);
//             match glob::glob_with(
//                 &path.to_string_lossy(),
//                 glob::MatchOptions {
//                     require_literal_leading_dot: true,
//                     ..Default::default()
//                 },
//             ) {
//                 Ok(files) => {
//                     for file in files {
//                         match file {
//                             Ok(ref f) => {
//                                 // It is not appropriate to try and remove the
//                                 // current directory or its parent when using
//                                 // glob patterns.
//                                 let name = f.display().to_string();
//                                 if name.ends_with("/.") || name.ends_with("/..") {
//                                     continue;
//                                 }

//                                 all_targets
//                                     .entry(f.clone())
//                                     .or_insert_with(|| target.tag.clone());
//                             }
//                             Err(e) => {
//                                 return Err(ShellError::labeled_error(
//                                     format!("Could not remove {:}", path.to_string_lossy()),
//                                     e.to_string(),
//                                     &target.tag,
//                                 ));
//                             }
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     return Err(ShellError::labeled_error(
//                         e.to_string(),
//                         e.to_string(),
//                         &name_tag,
//                     ))
//                 }
//             };
//         }

//         if all_targets.is_empty() && !_force {
//             return Err(ShellError::labeled_error(
//                 "No valid paths",
//                 "no valid paths",
//                 name_tag,
//             ));
//         }

//         Ok(all_targets
//             .into_iter()
//             .map(move |(f, tag)| {
//                 let is_empty = || match f.read_dir() {
//                     Ok(mut p) => p.next().is_none(),
//                     Err(_) => false,
//                 };

//                 if let Ok(metadata) = f.symlink_metadata() {
//                     #[cfg(unix)]
//                     let is_socket = metadata.file_type().is_socket();
//                     #[cfg(unix)]
//                     let is_fifo = metadata.file_type().is_fifo();

//                     #[cfg(not(unix))]
//                     let is_socket = false;
//                     #[cfg(not(unix))]
//                     let is_fifo = false;

//                     if metadata.is_file()
//                         || metadata.file_type().is_symlink()
//                         || recursive
//                         || is_socket
//                         || is_fifo
//                         || is_empty()
//                     {
//                         let result;
//                         #[cfg(feature = "trash-support")]
//                         {
//                             use std::io::Error;
//                             result = if _trash || (rm_always_trash && !_permanent) {
//                                 trash::delete(&f).map_err(|e: trash::Error| {
//                                     Error::new(ErrorKind::Other, format!("{:?}", e))
//                                 })
//                             } else if metadata.is_file() {
//                                 std::fs::remove_file(&f)
//                             } else {
//                                 std::fs::remove_dir_all(&f)
//                             };
//                         }
//                         #[cfg(not(feature = "trash-support"))]
//                         {
//                             result = if metadata.is_file() || is_socket || is_fifo {
//                                 std::fs::remove_file(&f)
//                             } else {
//                                 std::fs::remove_dir_all(&f)
//                             };
//                         }

//                         if let Err(e) = result {
//                             let msg =
//                                 format!("Could not delete because: {:}\nTry '--trash' flag", e);
//                             Err(ShellError::labeled_error(msg, e.to_string(), tag))
//                         } else {
//                             let val = format!("deleted {:}", f.to_string_lossy()).into();
//                             Ok(ReturnSuccess::Value(val))
//                         }
//                     } else {
//                         let msg =
//                             format!("Cannot remove {:}. try --recursive", f.to_string_lossy());
//                         Err(ShellError::labeled_error(
//                             msg,
//                             "cannot remove non-empty directory",
//                             tag,
//                         ))
//                     }
//                 } else {
//                     let msg = format!("no such file or directory: {:}", f.to_string_lossy());
//                     Err(ShellError::labeled_error(
//                         msg,
//                         "no such file or directory",
//                         tag,
//                     ))
//                 }
//             })
//             .into_action_stream())
//     }
