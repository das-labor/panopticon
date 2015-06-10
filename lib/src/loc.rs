use std::cell::{RefMut,RefCell};
use std::ops::DerefMut;
use marshal::{Marshal,Storage,Archive};
use uuid::Uuid;
use std::sync::Arc;

pub struct Loc<T: Marshal> {
    tag: Uuid,
    value: RefCell<Option<T>>,
    store: Option<Arc<Storage>>,
    previous: RefCell<Option<Archive>>,
}

impl<T: Marshal> Loc<T> {
    fn from_uuid(t: &Uuid, s: Arc<Storage>) -> Loc<T> {
        Loc::<T> {
            tag: t.clone(),
            value: None,
            store: Some(s),
            previous: None
        }
    }

    fn from_val(t: T) -> Loc<T> {
        Loc::<T> {
            tag: Uuid::new_v4(),
            value: t,
            store: None,
            previous: None,
        }
    }

    fn commit(&mut self, store: &Storage) -> bool {
        if let Some(t) = self.value.borrow().deref() {
            let &mut maybe_a = self.previous.borrow_mut();

            if let Some(ref a) = maybe_a {
                //a.statements.drain().inspect(store.remove);
                //a.blobs.drain().inspect(store.unregister);
            }

            let mut na = t.marshal();
            na.statements.drain().inspect(store.insert);
            na.blobs.drain().inspect(store.register);

            maybe_a = None;
            true
        } else {
            true
        }
    }

    fn read(&self) -> &T {
        match self.value.borrow_mut() {
            None => {
                self.inner.set(T::unmarshal(self.tag,s.unwrap()));
                self.read()
            },
            Some(ref v) => v
        }
    }

    fn write(&mut self) -> &mut T {
        match self.inner.borrow_mut() {
            LocInner::<T>::Location{ tag: t, store: s } => {
                self.inner.set(T::unmarshal(t,s.deref()));
                self.write()
            },
            LocInner::<T>::Value{ value: ref mut v,..} => v
        }
    }
}
