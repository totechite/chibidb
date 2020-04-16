use crate::storage::page::Page;
use std::collections::HashMap;
use crate::storage::util::{gen_hash, PageAuxiliar};
use std::rc::Rc;


// G-CLOCK(GeneralizeCLOCK)はLRUの近似アルゴリズムとして知られている。
#[derive(Default, Debug)]
struct G_CLOCK {
    hand_pointer: usize,
    evict_list: Vec<(u64, usize)>,
    items: HashMap<u64, PageAuxiliar>,
}

#[derive(Default, Debug)]
pub struct BufferPool {
    g_clock: G_CLOCK,
}

impl G_CLOCK {
    pub fn new() { Default::default() }
}

impl G_CLOCK {
    pub fn get(&mut self, hash: &u64) -> Option<&PageAuxiliar> {
        self.move_hand();
        if let Some(item) = self.items.get(hash) {
            Some(item)
        } else { None }
    }

    pub fn put(&mut self, pa: &PageAuxiliar) {
        let hash = gen_hash(&(pa.table_id + pa.page_id as u64));
        self.evict_list.push((hash, 0));
        self.items.insert(hash, pa.clone());
    }
}

impl G_CLOCK {
    fn move_hand(&mut self) {
        match self.evict_list.get(self.hand_pointer) {
            Some(&(hash, counter)) => {
                if counter > 0 {
                    self.evict_list[self.hand_pointer].1 -= 1;
                } else {
                    self.items.remove(&hash);
                    self.evict_list.remove(self.hand_pointer);
                }
                self.hand_pointer += 1;
            }
            None => {
                if self.evict_list.len() <= self.hand_pointer {
                    self.hand_pointer = 0;
                }
            }
        }
    }
}

impl BufferPool {
    pub fn new() -> Self { Default::default() }
}

impl BufferPool {
    pub fn read_page(&mut self, table_id: u64, page_id: u16) -> Option<&PageAuxiliar> {
        let hash = gen_hash(&(table_id + page_id as u64));
        self.g_clock.get(&hash)
    }
}
