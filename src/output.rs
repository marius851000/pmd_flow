use crate::{FlowData, FlowDataValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum OutputEnum {
    FollowGroup(FollowGroup),
    ScenarioWithBranch(ScenarioWithBranch),
    Scenario(Scenario),
    Dungeon(Dungeon),
    DungeonEnd(DungeonEnd),
    AskSave(AskSave),
    FreeMove(FreeMove),
    DgFlowBranchSetCounter(DgFlowBranchSetCounter),
    DgFlowBranch(DgFlowBranch),
    DgStagingPost(DgStagingPost),
    ScenarioWithProgNo(ScenarioWithProgNo),
    FreeMoveEvent(FreeMoveEvent),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Floor {
    i: String,
    o: String,
}

impl Floor {
    fn new(source: &FlowData, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let o = &dic["out"];
        let i = dic["in"].get_string().unwrap();
        let o = match o {
            FlowDataValue::String(str) => str.clone(),
            FlowDataValue::RefVec(vecid) => {
                let vec = source.get_vector(*vecid as usize).unwrap();
                vec[0].get_string().unwrap()
            }
            _ => panic!(),
        };

        Self { i, o }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dungeon {
    flowtype: String,
    comment: String,
    scenario_progress_no: String,
    socket: FollowSocket,
    party: Option<Vec<String>>,
    fixed_party_label: String,
    dungeon: String,
    layout: Layout,
    floor: Floor,
    debugname: String,
    debugmenu_tag: String,
}

impl Dungeon {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let flowtype = dic["flowtype"].get_string().unwrap();
        let comment = dic["comment"].get_string().unwrap();
        let scenario_progress_no = dic["scenarioProgressNo"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["start"],
            &["SEL_*", "next", "ok", "repeat", "select*"],
        );
        let party = match dic["party"] {
            FlowDataValue::String(_) => None,
            FlowDataValue::RefVec(vecid) => Some (
                source
                .get_vector(vecid as usize)
                .unwrap()
                .iter()
                .map(|x| x.get_string().unwrap())
                .collect()
            ),
            _ => panic!(),
        };
        let fixed_party_label = dic["fixed_party_label"].get_string().unwrap();
        let dungeon = dic["dungeon"].get_string().unwrap();
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let floor = Floor::new(source, dic["floor"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();

        Self {
            flowtype,
            comment,
            scenario_progress_no,
            socket,
            party,
            fixed_party_label,
            dungeon,
            layout,
            floor,
            debugname,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AskSave {
    comment: String,
    r#type: String,
    socket: FollowSocket,
    layout: Layout,
    debugname: String,
    debugmenu_tag: String,
}

impl AskSave {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let r#type = dic["type"].get_string().unwrap();
        let comment = dic["comment"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["in"],
            &["out"],
        );
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();

        Self {
            comment,
            r#type,
            socket,
            layout,
            debugname,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreeMoveEvent {
    comment: String,
    socket: FollowSocket,
    event_type: String,
    layout: Layout,
    debugname: String,
    debugmenu_tag: String,
}

impl FreeMoveEvent {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let comment = dic["comment"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["in"],
            &["out"],
        );
        let event_type = dic["eventType"].get_string().unwrap();
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();

        Self {
            comment,
            socket,
            event_type,
            layout,
            debugname,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DungeonEnd {
    comment: String,
    socket: FollowSocket,
    layout: Layout,
    debugname: String,
    debugmenu_tag: String,
}

impl DungeonEnd {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let comment = dic["comment"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["in"],
            &["out"],
        );
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();

        Self {
            comment,
            socket,
            layout,
            debugname,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowGroup {
    debug_groupname: String,
    data: Follow,
    layout: Layout,
    debugmenu_tag: String,
}

impl FollowGroup {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let debug_groupname = dic["debug_groupname"].get_string().unwrap();
        let data = Follow::new(
            source,
            tempory,
            source.get_vector(dic["data"].get_vecid().unwrap()).unwrap(),
        );
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();
        Self {
            debug_groupname,
            data,
            layout,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layout {
    line_break: bool,
    h: i64,
    w: i64,
    x: i64,
}

impl Layout {
    fn new(source: &FlowData, dicid: usize) -> Self {
        let layout_dic = source.get_dictionary(dicid).unwrap();
        let pos_dic = source
            .get_dictionary(layout_dic["layoutPos"].get_dicid().unwrap())
            .unwrap();
        Self {
            line_break: match layout_dic["lineBreak"].get_string().unwrap().as_str() {
                "true" => true,
                "false" => false,
                _ => panic!(),
            },
            h: pos_dic["posH"].get_string().unwrap().parse().unwrap(),
            w: pos_dic["posW"].get_string().unwrap().parse().unwrap(),
            x: pos_dic["posX"].get_string().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScenarioWithBranch {
    entry: Vec<String>,
    comment: String,
    socket: FollowSocket,
    branch: Vec<String>,
    layout: Layout,
    debugname: String,
    debugmenu_tag: String,
}

impl ScenarioWithBranch {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let entry = source
            .get_vector(dic["entry"].get_vecid().unwrap())
            .unwrap()
            .iter()
            .map(|x| x.get_string().unwrap().clone())
            .collect();
        let comment = dic["comment"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["start"],
            &["SEL_*", "next", "ok", "repeat", "select*"],
        );
        let branch = source
            .get_vector(dic["branch"].get_vecid().unwrap())
            .unwrap()
            .iter()
            .map(|x| x.get_string().unwrap())
            .collect();
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();
        Self {
            entry,
            comment,
            socket,
            branch,
            layout,
            debugname,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScenarioWithProgNo {
    entry: Vec<String>,
    comment: String,
    socket: FollowSocket,
    layout: Layout,
    debugname: String,
    timeline: Timeline,
    debugmenu_tag: String,
    scenario_progress_no: String,
}

impl ScenarioWithProgNo {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let entry = source
            .get_vector(dic["entry"].get_vecid().unwrap())
            .unwrap()
            .iter()
            .map(|x| x.get_string().unwrap().clone())
            .collect();
        let comment = dic["comment"].get_string().unwrap();
        let scenario_progress_no = dic["scenarioProgressNo"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["start"],
            &["next"],
        );
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let timeline = Timeline::new(source, dic["timeline"].get_dicid().unwrap());
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();
        Self {
            entry,
            comment,
            socket,
            layout,
            debugname,
            timeline,
            debugmenu_tag,
            scenario_progress_no,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timeline {
    dic: HashMap<String, String>,
}

impl Timeline {
    fn new(source: &FlowData, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let mut new_dic = HashMap::new();
        for key in dic.keys() {
            new_dic.insert(key.clone(), dic[key].get_string().unwrap());
        }
        Self { dic: new_dic }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreeMove {
    start_map: String,
    start_continue: String,
    start_place: String,
    comment: String,
    scenario_progress_no: String,
    next_cond_next: String,
    next_cond_other: String,
    follow_chara: Vec<String>,
    socket: FollowSocket,
    play_btn: String,
    layout: Layout,
    debugname: String,
    timeline: Timeline,
    debugmenu_tag: String,
}

impl FreeMove {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();

        let dic_start = source
            .get_dictionary(dic["start"].get_dicid().unwrap())
            .unwrap();
        let start_map = dic_start["map"].get_string().unwrap();
        let start_continue = dic_start["continue"].get_string().unwrap();
        let start_place = dic_start["place"].get_string().unwrap();
        let comment = dic["comment"].get_string().unwrap();
        let scenario_progress_no = dic["scenarioProgressNo"].get_string().unwrap(); //TODO some more code in the armoredmephit code
        let dic_next_cond = source
            .get_dictionary(dic["next_cond"].get_dicid().unwrap())
            .unwrap();
        let next_cond_next = dic_next_cond["next"].get_string().unwrap();
        let next_cond_other = match &dic_next_cond["other"] {
            FlowDataValue::String(str) => str.clone(),
            FlowDataValue::RefVec(vecid) => source.get_vector(*vecid as usize).unwrap()[0]
                .get_string()
                .unwrap(),
            _ => panic!(),
        };
        let follow_chara_dic = source
            .get_dictionary(dic["followChara"].get_dicid().unwrap())
            .unwrap();
        let mut follow_chara = Vec::new();
        for follow_chara_id in ["follow0", "follow1", "follow2"].iter() {
            let temp = follow_chara_dic[*follow_chara_id].clone();
            follow_chara.push(temp.get_string().unwrap());
        }
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["start"],
            &["next", "other0"],
        );
        let play_btn = dic["playBtn"].get_string().unwrap();
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let timeline = Timeline::new(source, dic["timeline"].get_dicid().unwrap());
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();

        FreeMove {
            start_map,
            start_continue,
            start_place,
            comment,
            scenario_progress_no,
            next_cond_next,
            next_cond_other,
            follow_chara,
            socket,
            play_btn,
            layout,
            debugname,
            timeline,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DgStagingPost {
    map: String,
    comment: String,
    socket: FollowSocket,
    layout: Layout,
    debugname: String,
    timeline: Timeline,
    debugmenu_tag: String,
}

impl DgStagingPost {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();

        let comment = dic["comment"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["start"],
            &["next", "other0"],
        );
        let map = dic["map"].get_string().unwrap();
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let timeline = Timeline::new(source, dic["timeline"].get_dicid().unwrap());
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();

        Self {
            comment,
            socket,
            map,
            layout,
            debugname,
            timeline,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DgFlowBranch {
    act: String,
    comment: String,
    count: String,
    socket: FollowSocket,
    r#if: String,
    id: String,
    layout: Layout,
    debugname: String,
    debugmenu_tag: String,
}

impl DgFlowBranch {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let act = dic["act"].get_string().unwrap();
        let comment = dic["comment"].get_string().unwrap();
        let count = dic["count"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["in", "reset_zero"],
            &["flow_A", "flow_B"],
        );
        let r#if = dic["if"].get_string().unwrap();
        let id = dic["id"].get_string().unwrap();
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();
        Self {
            act,
            comment,
            count,
            socket,
            r#if,
            id,
            layout,
            debugname,
            debugmenu_tag,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scenario {
    flowtype: String,
    entry: Vec<String>,
    comment: String,
    socket: FollowSocket,
    layout: Layout,
    debugname: String,
    debugmenu_tag: String,
}

impl Scenario {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let flowtype = dic["flowtype"].get_string().unwrap();
        let entry = source
            .get_vector(dic["entry"].get_vecid().unwrap())
            .unwrap()
            .iter()
            .map(|x| x.get_string().unwrap().clone())
            .collect();
        let comment = dic["comment"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["start"],
            &["next"],
        );
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();
        Self {
            flowtype,
            entry,
            comment,
            socket,
            layout,
            debugname,
            debugmenu_tag,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DgFlowBranchSetCounter {
    comment: String,
    count: String,
    socket: FollowSocket,
    id: String,
    layout: Layout,
    debugname: String,
    debugmenu_tag: String,
}

impl DgFlowBranchSetCounter {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, dicid: usize) -> Self {
        let dic = source.get_dictionary(dicid).unwrap();
        let comment = dic["comment"].get_string().unwrap();
        let count = dic["count"].get_string().unwrap();
        let socket = FollowSocket::new(
            source,
            tempory,
            dic["socket"].get_vecid().unwrap(),
            dicid,
            &["in"],
            &["out"],
        );
        let id = dic["id"].get_string().unwrap();
        let layout = Layout::new(source, dic["layout"].get_dicid().unwrap());
        let debugname = dic["debugname"].get_string().unwrap();
        let debugmenu_tag = dic["debugmenu_tag"].get_string().unwrap();
        Self {
            comment,
            count,
            socket,
            layout,
            id,
            debugname,
            debugmenu_tag,
        }
    }
}

fn follow_incoming_link(
    source: &FlowData,
    tempory: &mut FlowDataTempory,
    dicid: usize,
    parent_dicid: usize,
    valid_in_label: &[&str],
) -> HashMap<String, HashMap<String, String>> {
    let dic = source.get_dictionary(dicid).unwrap();
    let idn = dic["idname"].get_string().unwrap();
    let inl = dic["in"].get_string().unwrap();
    tempory
        .idname_set
        .insert(idn.clone(), (inl.clone(), parent_dicid));
    let mut result = HashMap::new();
    result.insert("%IN".into(), {
        let mut temp = HashMap::new();
        temp.insert(inl, idn);
        temp
    });
    result
}

fn follow_outgoing_link(
    source: &FlowData,
    tempory: &mut FlowDataTempory,
    dicid: usize,
    parent_dicid: usize,
    valid_out_label: &[&str],
) -> HashMap<String, String> {
    let dic = source.get_dictionary(dicid).unwrap();
    let label = dic["out"].get_string().unwrap();
    let dest = dic["to"].get_string().unwrap();
    let mut result = HashMap::new();
    result.insert(label, dest);
    result
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FollowSocket {
    socket_in: HashMap<String, String>,
    socket_out: HashMap<String, String>,
}

impl FollowSocket {
    fn new(
        source: &FlowData,
        tempory: &mut FlowDataTempory,
        vecid: usize,
        parent_dicid: usize,
        valid_in_label: &[&str],
        valid_out_label: &[&str],
    ) -> Self {
        let vec = source.get_vector(vecid).unwrap();
        let mut socket_out: HashMap<String, String> = HashMap::new();
        let mut socket_in: HashMap<String, String> = HashMap::new();

        for dicid in vec.iter().map(|x| x.get_dicid().unwrap()) {
            let dic = source.get_dictionary(dicid).unwrap();
            if dic.contains_key("idname") {
                let link =
                    follow_incoming_link(source, tempory, dicid, parent_dicid, valid_in_label);
                let link_in = &link["%IN"];
                for key in link_in.keys() {
                    socket_in.insert(key.clone(), link_in[key].clone());
                }
            } else if dic.contains_key("out") {
                let link =
                    follow_outgoing_link(source, tempory, dicid, parent_dicid, valid_out_label);
                for key in link.keys() {
                    socket_out.insert(key.clone(), link[key].clone());
                }
            } else {
                panic!();
            }
        }

        FollowSocket {
            socket_in,
            socket_out,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Follow {
    datas: Vec<OutputEnum>,
}

impl Follow {
    fn new(source: &FlowData, tempory: &mut FlowDataTempory, top_vec: &Vec<FlowDataValue>) -> Self {
        if top_vec.len() < 1 {
            panic!();
        };
        let mut datas = Vec::new();
        for value in top_vec {
            if let FlowDataValue::RefDic(dicid) = value {
                let dic = source.get_dictionary(*dicid as usize).unwrap();
                if dic.len() != 1 {
                    panic!();
                };
                let key = dic.keys().next().unwrap();
                let dicid = match dic[key] {
                    FlowDataValue::RefDic(refid) => refid,
                    _ => panic!(),
                } as usize;
                let to_add = match key.as_str() {
                    "Group" => OutputEnum::FollowGroup(FollowGroup::new(source, tempory, dicid)),
                    "ScenarioWithBranch" => OutputEnum::ScenarioWithBranch(
                        ScenarioWithBranch::new(source, tempory, dicid),
                    ),
                    "Scenario" => OutputEnum::Scenario(Scenario::new(source, tempory, dicid)),
                    "Dungeon" => OutputEnum::Dungeon(Dungeon::new(source, tempory, dicid)),
                    "DungeonEnd" => OutputEnum::DungeonEnd(DungeonEnd::new(source, tempory, dicid)),
                    "AskSave" => OutputEnum::AskSave(AskSave::new(source, tempory, dicid)),
                    "FreeMove" => OutputEnum::FreeMove(FreeMove::new(source, tempory, dicid)),
                    "DgFlowBranchSetCounter" => OutputEnum::DgFlowBranchSetCounter(
                        DgFlowBranchSetCounter::new(source, tempory, dicid),
                    ),
                    "DgFlowBranch" => {
                        OutputEnum::DgFlowBranch(DgFlowBranch::new(source, tempory, dicid))
                    }
                    "DgStagingPost" => {
                        OutputEnum::DgStagingPost(DgStagingPost::new(source, tempory, dicid))
                    }
                    "ScenarioWithProgNo" => OutputEnum::ScenarioWithProgNo(
                        ScenarioWithProgNo::new(source, tempory, dicid),
                    ),
                    "FreeMoveEvent" => OutputEnum::FreeMoveEvent(
                        FreeMoveEvent::new(source, tempory, dicid),
                    ),
                    unreconized => panic!("unreconized value: {:?}", unreconized),
                };
                println!("{:?}", to_add);
                datas.push(to_add);
            } else {
                panic!()
            }
        }
        Self { datas }
    }
}

#[derive(Default)]
struct FlowDataTempory {
    pub idname_set: HashMap<String, (String, usize)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlowDataOutput {
    output: Follow,
}

impl FlowDataOutput {
    pub fn new(source: FlowData) -> FlowDataOutput {
        let top_vec = source.get_vector(source.vector_len() - 1).unwrap();
        let mut tempory = FlowDataTempory::default();
        let output = Follow::new(&source, &mut tempory, top_vec);
        FlowDataOutput {output}
    }
}