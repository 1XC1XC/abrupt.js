mod crypto;
mod file;
mod rand;

#[inline(always)]
fn move_namespace(
    exports: &mut napi::JsObject,
    parent: &str,
    child: &str,
) -> napi::Result<()> {
    if !exports.has_named_property(parent)? || !exports.has_named_property(child)? {
        return Ok(());
    }

    let child_obj: napi::JsObject = exports.get_named_property(child)?;
    let mut parent_obj: napi::JsObject = exports.get_named_property(parent)?;
    parent_obj.set_named_property(child, child_obj)?;
    let _ = exports.delete_named_property(child)?;
    Ok(())
}

#[napi_derive::module_exports]
fn module_exports(mut exports: napi::JsObject) -> napi::Result<()> {
    move_namespace(&mut exports, "crypto", "base64")?;
    move_namespace(&mut exports, "crypto", "base16")?;
    move_namespace(&mut exports, "crypto", "base32")?;
    move_namespace(&mut exports, "crypto", "AES")?;
    move_namespace(&mut exports, "crypto", "RSA")?;
    move_namespace(&mut exports, "crypto", "morse")?;
    Ok(())
}
