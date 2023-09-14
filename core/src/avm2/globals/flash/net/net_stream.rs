use crate::avm2::error::type_error;
use crate::avm2::parameters::ParametersExt;
use crate::avm2::{Activation, Error, Object, TObject, Value};

pub use crate::avm2::object::netstream_allocator as net_stream_allocator;

pub fn get_bytes_loaded<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        return Ok(ns.bytes_loaded().into());
    }

    Ok(Value::Undefined)
}

pub fn get_bytes_total<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        return Ok(ns.bytes_total().into());
    }

    Ok(Value::Undefined)
}

pub fn play<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        let name = args
            .get(0)
            .cloned()
            .filter(|v| !matches!(v, Value::Null))
            .map(|v| v.coerce_to_string(activation))
            .transpose()?;

        ns.play(&mut activation.context, name);
    }

    Ok(Value::Undefined)
}

pub fn pause<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        ns.pause(&mut activation.context, true);
    }

    Ok(Value::Undefined)
}

pub fn resume<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        ns.resume(&mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn toggle_pause<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        ns.toggle_paused(&mut activation.context);
    }

    Ok(Value::Undefined)
}

pub fn get_client<'gc>(
    _activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        return Ok(ns.client().expect("NetStream client should be set").into());
    }

    Ok(Value::Undefined)
}

pub fn set_client<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    let client = args.get_value(0).as_object();

    if let Some(client) = client {
        if let Some(ns) = this.as_netstream() {
            ns.set_client(activation.context.gc_context, client);
        }
    } else {
        return Err(Error::AvmError(type_error(
            activation,
            "Error #2004: One of the parameters is invalid.",
            2004,
        )?));
    }

    Ok(Value::Undefined)
}

pub fn seek<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Object<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    if let Some(ns) = this.as_netstream() {
        let offset = args
            .get(0)
            .cloned()
            .unwrap_or(Value::Undefined)
            .coerce_to_number(activation)?;
        ns.seek(&mut activation.context, offset);
    }

    Ok(Value::Undefined)
}
