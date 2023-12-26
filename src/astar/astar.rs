use std::cell::RefCell;
use std::sync::{Arc, RwLock};
//use tokio::fs::File;
//use tokio::io::AsyncReadExt;
use crate::errors::my_errors::RetResult;
use crate::map::{Map, OpenList, Point, PointType};

pub struct  AStar {
    pub map: Box<Vec<Vec<i32>>>,
}

impl Map for AStar {
    fn new() -> Arc<RwLock<dyn Map>> {
        Arc::new(RwLock::new(AStar { map: Box::new(vec![]) }))
    }

    fn load(&mut self, points: Vec<Vec<i32>>) -> RetResult<()> {
        self.map = Box::new(points);
        Ok(())
    }

    /*async fn load_from_file(&mut self, points: String) -> RetResult<()> {

        // 打开文件
        let file = File::open("path/to/your/map_file.txt").await?;

        // 创建一个缓冲区来存储文件内容
        let mut buffer = Vec::new();

        // 通过异步读取文件内容到缓冲区
        file.take(1024) // 读取文件的前 1024 字节，可以根据需要调整
            .read_to_end(&mut buffer)
            .await?;

        // 打印文件内容
        println!("{}", String::from_utf8_lossy(&buffer));

        Ok(())
    }*/

    fn find_path(&self, start: PointType, end: PointType) -> Vec<Point> {

        //let start = start.into_rc();
        //let end = end.into_rc();

        let open_list = RefCell::new(OpenList::new());
        let mut close_list = OpenList::new();

        let mut last = end.clone();

        open_list.borrow_mut().insert(start.clone());

        while open_list.borrow().len() > 0 {

            let min_f = open_list.borrow_mut().min_f();


            match min_f {
                None => break,
                Some(v) => {

                    let neighbors = v.borrow().neighbors(&start, &end);

                    for neighbor in neighbors.into_iter() {
                        if self.in_map(neighbor.clone())
                            && !close_list.contains_point(neighbor.clone()){

                            if !open_list.borrow().contains_point(neighbor.clone()) {
                                neighbor.borrow_mut().set_parent(v.clone());
                                open_list.borrow_mut().insert(neighbor.clone());
                                if neighbor == end {
                                    last = neighbor;
                                    break
                                }
                            } else {
                                let new_g = v.borrow().g + v.borrow().distance(neighbor.clone());
                                if new_g <= neighbor.borrow().g {
                                    let mut nmb = neighbor.borrow_mut();
                                    nmb.set_parent(v.clone());
                                    nmb.set_g(new_g);
                                }
                                last = neighbor;
                            }
                        }
                    }

                    open_list.borrow_mut().remove(v.clone());
                    close_list.insert(v.clone());

                }
            }

            if open_list.borrow().contains_point(end.clone()){
                break
            }
        }

        let mut list = vec![last.borrow().clone()];
        let mut p = last.borrow().clone().parent;
        while p != None {
            match p {
                None => break,
                Some(v) => {
                    list.insert(0, v.borrow().clone());
                    p = v.borrow_mut().parent.take()
                }
            }
        }

        list
    }

    fn in_map(&self, point: PointType) -> bool {
        let point = point.borrow();
        if point.x < 0 || point.y < 0 {return false}
        if point.x >= self.map.len() as i64 || point.y >= self.map[0].len()  as i64 {return false}
        if self.map[point.y as usize][point.x as usize] == 1 {return false}
        true
    }
}

impl Drop for AStar {
    fn drop(&mut self) {
        self.map.clear()
    }
}