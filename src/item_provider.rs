use fnv::FnvHashMap;
use my_node::*;
use nx;
use nx::GenericNode;
use std::mem;
use std::ops::Deref;
use std::path::Path;

#[derive(Debug)]
pub struct ItemData {
    pub id: i32,
    pub price: i32,
    pub max_per_slot: i16,
    pub trade_able: bool,
    pub is_cash: bool,

    // item effect data
    pub recover: bool,
    pub hp: i16,
    pub mp: i16,
    pub watk: i16,
    pub matk: i16,
    pub wdef: i16,
    pub mdef: i16,
    pub acc: i16,
    pub avo: i16,
    pub jump: i16,
    pub speed: i16,
    pub hhp: i16,
    pub hmp: i16,
    pub hpr: i16,
    pub mpr: i16,
    pub success: i16,
    pub cursed: i16,
    pub strr: i16,
    pub dex: i16,
    pub intt: i16,
    pub luk: i16,
    pub time: i32,
    pub move_to: i32,
}

impl ItemData {
    fn new() -> Self {
        // pod style struct
        let s: ItemData = unsafe { mem::zeroed() };
        s
    }
}

pub struct ItemProvider {
    map: FnvHashMap<i32, ItemData>,
}

impl ItemProvider {
    pub fn new() -> Self {
        let file = unsafe { nx::File::open(&Path::new(r"Item.nx")).unwrap() };
        let mut map: FnvHashMap<i32, ItemData> = FnvHashMap::default();
        load_cash(&mut map, file.root());
        load_consume(&mut map, file.root());
        load_etc(&mut map, file.root());
        load_install(&mut map, file.root());
        ItemProvider { map }
    }

    pub fn get_item_data(&self, item_id: i32) -> Option<&ItemData> {
        self.map.get(&item_id)
    }
}

fn load_cash<'a>(map: &mut FnvHashMap<i32, ItemData>, root: nx::Node<'a>) {
    let cash = root.get("Cash").expect("Failed to load Cash");
    //item/Cash/xx.img/xx_id/
    for xx_img in cash.iter() {
        for xx_id in xx_img.iter() {
            let mut cash_item = ItemData::new();
            cash_item.id = xx_id.name().parse().unwrap();
            cash_item.max_per_slot = 100;
            cash_item.trade_able = true;
            cash_item.is_cash = true;
            map.insert(cash_item.id, cash_item);
        }
    }
}

fn load_consume<'a>(map: &mut FnvHashMap<i32, ItemData>, root: nx::Node<'a>) {
    let consume = root.get("Consume").expect("Failed to load Consume");
    //item/Consume/xx.img/xx_id/info
    for xx_img in consume.iter() {
        for xx_id in xx_img.iter() {
            let mut consume_item = ItemData::new();
            consume_item.id = xx_id.name().parse().unwrap();
            consume_item.max_per_slot = 100;
            consume_item.trade_able = true;

            if let Some(info) = xx_id.get("info") {
                for i in info.iter() {
                    let i = MyNode(i);
                    match i.deref().name() {
                        "slotMax" => consume_item.max_per_slot = i.get_i16(),
                        "price" => consume_item.price = i.get_i32(),
                        "tradeBlock" => consume_item.trade_able = i.get_i16() != 0,
                        _ => continue,
                    }
                }
            }

            // scroll data
            if let Some(info) = xx_id.get("info") {
                for i in info.iter() {
                    let i = MyNode(i);
                    match i.deref().name() {
                        "success" => consume_item.success = i.get_i16(),
                        "cursed" => consume_item.cursed = i.get_i16(),
                        "recover" => consume_item.recover = i.get_i16() != 0,
                        "incJump" => consume_item.jump = i.get_i16(),
                        "incSpeed" => consume_item.speed = i.get_i16(),
                        "incSTR" => consume_item.strr = i.get_i16(),
                        "incDEX" => consume_item.dex = i.get_i16(),
                        "incINT" => consume_item.intt = i.get_i16(),
                        "incLUK" => consume_item.luk = i.get_i16(),
                        "incMHP" => consume_item.hp = i.get_i16(),
                        "incMMP" => consume_item.mp = i.get_i16(),
                        "incEVA" => consume_item.avo = i.get_i16(),
                        "incACC" => consume_item.acc = i.get_i16(),
                        "incPAD" => consume_item.watk = i.get_i16(),
                        "incMAD" => consume_item.matk = i.get_i16(),
                        "incPDD" => consume_item.wdef = i.get_i16(),
                        "incMDD" => consume_item.mdef = i.get_i16(),
                        _ => continue,
                    }
                }
            }

            if let Some(spec) = xx_id.get("spec") {
                for s in spec.iter() {
                    let s = MyNode(s);
                    match s.deref().name() {
                        "hpR" => consume_item.hpr = s.get_i16(),
                        "mpR" => consume_item.mpr = s.get_i16(),
                        "time" => consume_item.time = s.get_i32(),
                        "hp" => consume_item.hp = s.get_i16(),
                        "mp" => consume_item.mp = s.get_i16(),
                        "acc" => consume_item.acc = s.get_i16(),
                        "eva" => consume_item.avo = s.get_i16(),
                        "speed" => consume_item.speed = s.get_i16(),
                        "jump" => consume_item.jump = s.get_i16(),
                        "pdd" => consume_item.wdef = s.get_i16(),
                        "mdd" => consume_item.mdef = s.get_i16(),
                        "pad" => consume_item.watk = s.get_i16(),
                        "mad" => consume_item.matk = s.get_i16(),
                        "moveTo" => consume_item.move_to = s.get_i32(),
                        _ => continue,
                    }
                }
            }

            map.insert(consume_item.id, consume_item);
        }
    }
}

fn load_etc<'a>(map: &mut FnvHashMap<i32, ItemData>, root: nx::Node<'a>) {
    let etc = root.get("Etc").expect("Failed to load Etc");
    //item/Etc/xx.img/xx_id/
    for xx_img in etc.iter() {
        for xx_id in xx_img.iter() {
            let mut etc_item = ItemData::new();
            etc_item.id = xx_id.name().parse().unwrap();
            etc_item.max_per_slot = 100;
            etc_item.trade_able = true;

            if let Some(info) = xx_id.get("info") {
                for i in info.iter() {
                    let i = MyNode(i);
                    match i.deref().name() {
                        "slotMax" => etc_item.max_per_slot = i.get_i16(),
                        "price" => etc_item.price = i.get_i32(),
                        "tradeBlock" => etc_item.trade_able = i.get_i16() != 0,
                        _ => continue,
                    }
                }
            }

            map.insert(etc_item.id, etc_item);
        }
    }
}

fn load_install<'a>(map: &mut FnvHashMap<i32, ItemData>, root: nx::Node<'a>) {
    let etc = root.get("Install").expect("Failed to load Install");
    //item/Install/xx.img/xx_id/
    for xx_img in etc.iter() {
        for xx_id in xx_img.iter() {
            let mut etc_item = ItemData::new();
            etc_item.id = xx_id.name().parse().unwrap();
            etc_item.max_per_slot = 100;
            etc_item.trade_able = true;

            if let Some(info) = xx_id.get("info") {
                for i in info.iter() {
                    let i = MyNode(i);
                    match i.deref().name() {
                        "slotMax" => etc_item.max_per_slot = i.get_i16(),
                        "price" => etc_item.price = i.get_i32(),
                        "tradeBlock" => etc_item.trade_able = i.get_i16() != 0,
                        _ => continue,
                    }
                }
            }

            map.insert(etc_item.id, etc_item);
        }
    }
}
