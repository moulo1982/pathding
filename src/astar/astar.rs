use std::sync::{Arc, RwLock};
//use tokio::fs::File;
//use tokio::io::AsyncReadExt;
use crate::errors::my_errors::RetResult;
use crate::map::{Map, List, Point, PointType};

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

        let mut open_list = List::new();
        let mut close_list = List::new();

        let mut last = end.clone();

        open_list.insert(start.clone());

        while open_list.len() > 0 {

            let min_f = open_list.min_f();
            
            match min_f {
                None => break,
                Some(v) => {

                    let neighbors = v.borrow().neighbors(&start, &end);

                    for neighbor in neighbors.into_iter() {
                        if self.in_map(&neighbor)
                            && !close_list.contains_point(&neighbor){

                            if !open_list.contains_point(&neighbor) {
                                neighbor.borrow_mut().set_parent(v.clone());
                                open_list.insert(neighbor.clone());
                                if neighbor == end {
                                    last = neighbor;
                                    break
                                }
                            } else {
                                let new_g = v.borrow().g + v.borrow().distance(&neighbor);
                                if new_g <= neighbor.borrow().g {
                                    open_list.remove(&neighbor);
                                    {
                                        let mut nmb = neighbor.borrow_mut();
                                        nmb.set_parent(v.clone());
                                        nmb.set_g(new_g);
                                    }
                                    open_list.insert(neighbor.clone());
                                }
                                last = neighbor;
                            }
                        }
                    }

                    open_list.remove(&v);
                    close_list.insert(v);

                }
            }

            if open_list.contains_point(&end){
                break
            }
        }

        let mut lb = last.borrow_mut();
        let mut list = vec![lb.clone()];
        let mut p = lb.parent.take();
        while p != None {
            match p {
                None => break,
                Some(v) => {
                    list.push(v.borrow().clone());
                    p = v.borrow_mut().parent.take()
                }
            }
        }

        list
    }

    fn in_map(&self, point: &PointType) -> bool {
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