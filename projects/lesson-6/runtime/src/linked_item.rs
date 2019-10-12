use support::{StorageMap, Parameter};
use sr_primitives::traits::Member;
use codec::{Encode, Decode};

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct LinkedItem<Value> {
	pub prev: Option<Value>,
	pub next: Option<Value>,
}

pub struct LinkedList<Storage, Key, Value>(rstd::marker::PhantomData<(Storage, Key, Value)>);

impl<Storage, Key, Value> LinkedList<Storage, Key, Value> where
    Value: Parameter + Member + Copy,
    Key: Parameter,
    Storage: StorageMap<(Key, Option<Value>), LinkedItem<Value>, Query = Option<LinkedItem<Value>>>,
{
    fn read_head(key: &Key) -> LinkedItem<Value> {
 		Self::read(key, None)
 	}

  	fn write_head(account: &Key, item: LinkedItem<Value>) {
 		Self::write(account, None, item);
 	}

  	fn read(key: &Key, value: Option<Value>) -> LinkedItem<Value> {
 		Storage::get(&(key.clone(), value)).unwrap_or_else(|| LinkedItem {
 			prev: None,
 			next: None,
 		})
 	}

  	fn write(key: &Key, value: Option<Value>, item: LinkedItem<Value>) {
 		Storage::insert(&(key.clone(), value), item);
 	}

    pub fn append(key: &Key, value: Value) {
        // 作业：实现 append
				let mut head = Self::read_head(&key);
				let mut last = Self::read(&key,head.prev);
				let new_item = LinkedItem {
					prev: head.prev,
					next: None,
				};
				let old_prev = head.prev;
				head.prev = Some(value);
				// 第一个元素的话
				if head.next == None {
					head.next = Some(value);
				}
				last.next = Some(value);
				Self::write(&key,old_prev,last);
				Self::write(&key,None,head);

				Self::write(&key,Some(value),new_item);

    }

    pub fn remove(key: &Key, value: Value) {
        // 作业：实现 remove
				let  item = Self::read(&key,Some(value));
				let mut prev_item = Self::read(&key,item.prev); 
				prev_item.next = item.next;
				Self::write(&key,item.prev,prev_item);

				let mut next_item = Self::read(&key,item.next); 
				next_item.prev = item.prev;
				Self::write(&key,item.next,next_item);
				Storage::remove(&(key.clone(), Some(value)));
    }
}