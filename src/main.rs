trait Table {
	type PrimaryKey;
	type Row;
	fn new() -> Self where Self:Sized;
	fn get_cloned(&self, key: &Self::PrimaryKey) -> Option<Self::Row> where Self::PrimaryKey: Ord + PartialOrd + PartialEq + Eq, Self::Row: Clone;
	fn select(&self) -> Vec<Self::Row>;
	//fn where(&self) {}
}

macro_rules! table {
	($table_name:ident { $($field_name:ident: $field_type:ty,)* #[primary] $pk_name:ident: $pk_type:ty}) => (
		#[derive(Debug)]
		struct $table_name {
			$(
				$field_name: Vec<$field_type>,
			)*
			$pk_name: Vec<$pk_type>
		}
		impl $table_name {
			fn insert(&mut self, $($field_name: $field_type,)* $pk_name: $pk_type) {
				$(
					self.$field_name.push($field_name);
				)*
				self.$pk_name.push($pk_name);
			}
			fn get(&self, pk: &$pk_type) -> Option<($(&$field_type,)* &$pk_type)> {
				for (i, item) in self.$pk_name.iter().enumerate() {
					if item == pk {
						return Some(($(&self.$field_name[i], )* &self.$pk_name[i]));
					}
				}
				None
			}
		}
		impl Table for $table_name {
			type PrimaryKey = $pk_type;
			type Row = ($($field_type,)* $pk_type);
			fn new() -> Self {
				$table_name{$($field_name: Vec::new(), )* $pk_name: Vec::new()}
			}
			fn get_cloned(&self, pk: &$pk_type) -> Option<Self::Row> where Self::Row: Clone {
				for (i, item) in self.$pk_name.iter().enumerate() {
					if item == pk {
						return Some(($(self.$field_name[i].clone(), )* self.$pk_name[i].clone()));
					}
				}
				None
			}
			fn select(&self) -> Vec<Self::Row> {
				let mut rows = Vec::new();
				for i in 0..self.$pk_name.len() {
					rows.push(($(self.$field_name[i],)* self.$pk_name[i]));
				}
				rows
			}
		}
	)
}

trait Where {
	fn select_where(&self) -> Vec<Self>;
}

table!{ExampleTable {
	x: i32,
	y: i32,
	#[primary]
	name: &'static str
}}

fn main() {
	let mut coords = ExampleTable::new();
	coords.insert(0, 1, "First!");
	coords.insert(1, 2, "Second!");
	coords.insert(-1, -1, "blah");
	println!("{:?}", coords.get(&"First!"));
	println!("{:?}", coords.get(&"blah"));
	println!("{:?}", coords.select().where(|k| k.2 == "blah"));
}
