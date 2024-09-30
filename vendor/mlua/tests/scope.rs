use std::cell::Cell;
use std::rc::Rc;
use std::string::String as StdString;
use std::sync::Arc;

use mlua::{
    AnyUserData, Error, Function, Lua, MetaMethod, Result, String, UserData, UserDataFields,
    UserDataMethods,
};

#[test]
fn test_scope_func() -> Result<()> {
    let lua = Lua::new();

    let rc = Rc::new(Cell::new(0));
    lua.scope(|scope| {
        let r = rc.clone();
        let f = scope.create_function(move |_, ()| {
            r.set(42);
            Ok(())
        })?;
        lua.globals().set("bad", f.clone())?;
        f.call::<_, ()>(())?;
        assert_eq!(Rc::strong_count(&rc), 2);
        Ok(())
    })?;
    assert_eq!(rc.get(), 42);
    assert_eq!(Rc::strong_count(&rc), 1);

    match lua.globals().get::<_, Function>("bad")?.call::<_, ()>(()) {
        Err(Error::CallbackError { ref cause, .. }) => match *cause.as_ref() {
            Error::CallbackDestructed => {}
            ref err => panic!("wrong error type {:?}", err),
        },
        r => panic!("improper return for destructed function: {:?}", r),
    };

    Ok(())
}

#[test]
fn test_scope_capture() -> Result<()> {
    let lua = Lua::new();

    let mut i = 0;
    lua.scope(|scope| {
        scope
            .create_function_mut(|_, ()| {
                i = 42;
                Ok(())
            })?
            .call::<_, ()>(())
    })?;
    assert_eq!(i, 42);

    Ok(())
}

#[test]
fn test_scope_outer_lua_access() -> Result<()> {
    let lua = Lua::new();

    let table = lua.create_table()?;
    lua.scope(|scope| {
        scope
            .create_function_mut(|_, ()| table.set("a", "b"))?
            .call::<_, ()>(())
    })?;
    assert_eq!(table.get::<_, String>("a")?, "b");

    Ok(())
}

#[test]
fn test_scope_userdata_fields() -> Result<()> {
    struct MyUserData<'a>(&'a Cell<i64>);

    impl<'a> UserData for MyUserData<'a> {
        fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
            fields.add_field("field", "hello");
            fields.add_field_method_get("val", |_, data| Ok(data.0.get()));
            fields.add_field_method_set("val", |_, data, val| {
                data.0.set(val);
                Ok(())
            });
        }
    }

    let lua = Lua::new();

    let i = Cell::new(42);
    let f: Function = lua
        .load(
            r#"
            function(u)
                assert(u.field == "hello")
                assert(u.val == 42)
                u.val = 44
            end
        "#,
        )
        .eval()?;

    lua.scope(|scope| f.call::<_, ()>(scope.create_nonstatic_userdata(MyUserData(&i))?))?;

    assert_eq!(i.get(), 44);

    Ok(())
}

#[test]
fn test_scope_userdata_methods() -> Result<()> {
    struct MyUserData<'a>(&'a Cell<i64>);

    impl<'a> UserData for MyUserData<'a> {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("inc", |_, data, ()| {
                data.0.set(data.0.get() + 1);
                Ok(())
            });

            methods.add_method("dec", |_, data, ()| {
                data.0.set(data.0.get() - 1);
                Ok(())
            });
        }
    }

    let lua = Lua::new();

    let i = Cell::new(42);
    let f: Function = lua
        .load(
            r#"
            function(u)
                u:inc()
                u:inc()
                u:inc()
                u:dec()
            end
        "#,
        )
        .eval()?;

    lua.scope(|scope| f.call::<_, ()>(scope.create_nonstatic_userdata(MyUserData(&i))?))?;

    assert_eq!(i.get(), 44);

    Ok(())
}

#[test]
fn test_scope_userdata_functions() -> Result<()> {
    struct MyUserData<'a>(&'a i64);

    impl<'a> UserData for MyUserData<'a> {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_meta_method(MetaMethod::Add, |lua, this, ()| {
                let globals = lua.globals();
                globals.set("i", globals.get::<_, i64>("i")? + this.0)?;
                Ok(())
            });
            methods.add_meta_method(MetaMethod::Sub, |lua, this, ()| {
                let globals = lua.globals();
                globals.set("i", globals.get::<_, i64>("i")? + this.0)?;
                Ok(())
            });
        }
    }

    let lua = Lua::new();

    let dummy = 1;
    let f = lua
        .load(
            r#"
            i = 0
            return function(u)
                _ = u + u
                _ = u - 1
                _ = u + 1
            end
        "#,
        )
        .eval::<Function>()?;

    lua.scope(|scope| f.call::<_, ()>(scope.create_nonstatic_userdata(MyUserData(&dummy))?))?;

    assert_eq!(lua.globals().get::<_, i64>("i")?, 3);

    Ok(())
}

#[test]
fn test_scope_userdata_mismatch() -> Result<()> {
    struct MyUserData<'a>(&'a Cell<i64>);

    impl<'a> UserData for MyUserData<'a> {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("inc", |_, data, ()| {
                data.0.set(data.0.get() + 1);
                Ok(())
            });
        }
    }

    let lua = Lua::new();

    lua.load(
        r#"
        function okay(a, b)
            a.inc(a)
            b.inc(b)
        end
        function bad(a, b)
            a.inc(b)
        end
    "#,
    )
    .exec()?;

    let a = Cell::new(1);
    let b = Cell::new(1);

    let okay: Function = lua.globals().get("okay")?;
    let bad: Function = lua.globals().get("bad")?;

    lua.scope(|scope| {
        let au = scope.create_nonstatic_userdata(MyUserData(&a))?;
        let bu = scope.create_nonstatic_userdata(MyUserData(&b))?;
        assert!(okay.call::<_, ()>((au.clone(), bu.clone())).is_ok());
        match bad.call::<_, ()>((au, bu)) {
            Err(Error::CallbackError { ref cause, .. }) => match cause.as_ref() {
                Error::BadArgument {
                    to,
                    pos,
                    name,
                    cause,
                } => {
                    assert_eq!(to.as_deref(), Some("MyUserData.inc"));
                    assert_eq!(*pos, 1);
                    assert_eq!(name.as_deref(), Some("self"));
                    assert!(matches!(*cause.as_ref(), Error::UserDataTypeMismatch));
                }
                other => panic!("wrong error type {:?}", other),
            },
            Err(other) => panic!("wrong error type {:?}", other),
            Ok(_) => panic!("incorrectly returned Ok"),
        }
        Ok(())
    })?;

    Ok(())
}

#[test]
fn test_scope_userdata_drop() -> Result<()> {
    let lua = Lua::new();

    struct MyUserData(#[allow(unused)] Rc<()>);

    impl UserData for MyUserData {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("method", |_, _, ()| Ok(()));
        }
    }

    struct MyUserDataArc(#[allow(unused)] Arc<()>);

    impl UserData for MyUserDataArc {}

    let rc = Rc::new(());
    let arc = Arc::new(());
    lua.scope(|scope| {
        let ud = scope.create_userdata(MyUserData(rc.clone()))?;
        ud.set_user_value(MyUserDataArc(arc.clone()))?;
        lua.globals().set("ud", ud)?;
        assert_eq!(Rc::strong_count(&rc), 2);
        assert_eq!(Arc::strong_count(&arc), 2);
        Ok(())
    })?;

    lua.gc_collect()?;
    assert_eq!(Rc::strong_count(&rc), 1);
    assert_eq!(Arc::strong_count(&arc), 1);

    match lua.load("ud:method()").exec() {
        Err(Error::CallbackError { ref cause, .. }) => match cause.as_ref() {
            Error::CallbackDestructed => {}
            err => panic!("expected CallbackDestructed, got {:?}", err),
        },
        r => panic!("improper return for destructed userdata: {:?}", r),
    };

    let ud = lua.globals().get::<_, AnyUserData>("ud")?;
    match ud.borrow::<MyUserData>() {
        Ok(_) => panic!("succesfull borrow for destructed userdata"),
        Err(Error::UserDataDestructed) => {}
        Err(err) => panic!("improper borrow error for destructed userdata: {:?}", err),
    }

    match ud.get_metatable() {
        Ok(_) => panic!("successful metatable retrieval of destructed userdata"),
        Err(Error::UserDataDestructed) => {}
        Err(err) => panic!(
            "improper metatable error for destructed userdata: {:?}",
            err
        ),
    }

    Ok(())
}

#[test]
fn test_scope_nonstatic_userdata_drop() -> Result<()> {
    let lua = Lua::new();

    struct MyUserData<'a>(&'a Cell<i64>, #[allow(unused)] Arc<()>);

    impl<'a> UserData for MyUserData<'a> {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("inc", |_, data, ()| {
                data.0.set(data.0.get() + 1);
                Ok(())
            });
        }
    }

    struct MyUserDataArc(#[allow(unused)] Arc<()>);

    impl UserData for MyUserDataArc {}

    let i = Cell::new(1);
    let arc = Arc::new(());
    lua.scope(|scope| {
        let ud = scope.create_nonstatic_userdata(MyUserData(&i, arc.clone()))?;
        ud.set_user_value(MyUserDataArc(arc.clone()))?;
        lua.globals().set("ud", ud)?;
        lua.load("ud:inc()").exec()?;
        assert_eq!(Arc::strong_count(&arc), 3);
        Ok(())
    })?;

    lua.gc_collect()?;
    assert_eq!(Arc::strong_count(&arc), 1);

    match lua.load("ud:inc()").exec() {
        Err(Error::CallbackError { ref cause, .. }) => match cause.as_ref() {
            Error::CallbackDestructed => {}
            err => panic!("expected CallbackDestructed, got {:?}", err),
        },
        r => panic!("improper return for destructed userdata: {:?}", r),
    };

    let ud = lua.globals().get::<_, AnyUserData>("ud")?;
    match ud.borrow::<MyUserData>() {
        Ok(_) => panic!("succesfull borrow for destructed userdata"),
        Err(Error::UserDataDestructed) => {}
        Err(err) => panic!("improper borrow error for destructed userdata: {:?}", err),
    }
    match ud.get_metatable() {
        Ok(_) => panic!("successful metatable retrieval of destructed userdata"),
        Err(Error::UserDataDestructed) => {}
        Err(err) => panic!(
            "improper metatable error for destructed userdata: {:?}",
            err
        ),
    }

    Ok(())
}

#[test]
fn test_scope_userdata_ref() -> Result<()> {
    let lua = Lua::new();

    struct MyUserData(Cell<i64>);

    impl UserData for MyUserData {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("inc", |_, data, ()| {
                data.0.set(data.0.get() + 1);
                Ok(())
            });

            methods.add_method("dec", |_, data, ()| {
                data.0.set(data.0.get() - 1);
                Ok(())
            });
        }
    }

    let data = MyUserData(Cell::new(1));
    lua.scope(|scope| {
        let ud = scope.create_userdata_ref(&data)?;
        modify_userdata(&lua, ud)
    })?;
    assert_eq!(data.0.get(), 2);

    Ok(())
}

#[test]
fn test_scope_userdata_ref_mut() -> Result<()> {
    let lua = Lua::new();

    struct MyUserData(i64);

    impl UserData for MyUserData {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method_mut("inc", |_, data, ()| {
                data.0 += 1;
                Ok(())
            });

            methods.add_method_mut("dec", |_, data, ()| {
                data.0 -= 1;
                Ok(())
            });
        }
    }

    let mut data = MyUserData(1);
    lua.scope(|scope| {
        let ud = scope.create_userdata_ref_mut(&mut data)?;
        modify_userdata(&lua, ud)
    })?;
    assert_eq!(data.0, 2);

    Ok(())
}

#[test]
fn test_scope_any_userdata() -> Result<()> {
    let lua = Lua::new();

    lua.register_userdata_type::<StdString>(|reg| {
        reg.add_meta_method("__tostring", |_, data, ()| Ok(data.clone()));
    })?;

    lua.scope(|scope| {
        let ud = scope.create_any_userdata(StdString::from("foo"))?;
        lua.globals().set("ud", ud)?;
        lua.load("assert(tostring(ud) == 'foo')").exec()
    })?;

    // Check that userdata is destructed
    match lua.load("tostring(ud)").exec() {
        Err(Error::CallbackError { ref cause, .. }) => match cause.as_ref() {
            Error::CallbackDestructed => {}
            err => panic!("expected CallbackDestructed, got {:?}", err),
        },
        r => panic!("improper return for destructed userdata: {:?}", r),
    };

    Ok(())
}

#[test]
fn test_scope_any_userdata_ref() -> Result<()> {
    let lua = Lua::new();

    lua.register_userdata_type::<Cell<i64>>(|reg| {
        reg.add_method("inc", |_, data, ()| {
            data.set(data.get() + 1);
            Ok(())
        });

        reg.add_method("dec", |_, data, ()| {
            data.set(data.get() - 1);
            Ok(())
        });
    })?;

    let data = Cell::new(1i64);
    lua.scope(|scope| {
        let ud = scope.create_any_userdata_ref(&data)?;
        modify_userdata(&lua, ud)
    })?;
    assert_eq!(data.get(), 2);

    Ok(())
}

fn modify_userdata(lua: &Lua, ud: AnyUserData) -> Result<()> {
    let f: Function = lua
        .load(
            r#"
    function(u)
        u:inc()
        u:dec()
        u:inc()
    end
"#,
        )
        .eval()?;

    f.call(ud)?;

    Ok(())
}
