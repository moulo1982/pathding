use std::cell::RefCell;
use std::sync::{Arc, RwLock};
//use tokio::fs::File;
//use tokio::io::AsyncReadExt;
use crate::errors::my_errors::RetResult;
use crate::map::{Map, OpenList, Point};

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

    fn find_path(&self, start: &Point, end: &Point) -> Vec<Point> {

        let open_list = RefCell::new(OpenList::new());
        let close_list = RefCell::new(OpenList::new());

        open_list.borrow_mut().insert(&start, &end, start.clone());

        while open_list.borrow().len() > 0 && !open_list.borrow().contains_point(end){

            let min_f = open_list.borrow_mut().min_f();
            match min_f {
                None => break,
                Some(v) => {
                    let neighbors = v.borrow().neighbors();
                    for mut one in neighbors {

                        if !self.in_map(&one) || open_list.borrow().contains_point(&one) || open_list.borrow().contains_point(&one) {
                            continue
                        }

                        one.set_parent(v.clone());
                        open_list.borrow_mut().insert(&start, &end, one);//one直接移动到函数内，插入到列表中，后面不用了
                    }

                    close_list.borrow_mut().insert(&start, &end, v.take());//min_f直接移动到函数内，插入到列表中，后面不用了

                    //没写完，直接跳出，避免死循环
                    break
                }
            }
        }

        let x = open_list.borrow().to_array(); x
    }

    fn in_map(&self, point:&Point) -> bool {
        let borrow = point;//.borrow();
        if borrow.x < 0 || borrow.y < 0 {return false}
        if borrow.x > self.map.len() as i64 || borrow.x > self.map[0].len()  as i64 {return false}
        if self.map[borrow.x as usize][borrow.y as usize] == 1 {return false}
        true
    }
}

impl Drop for AStar {
    fn drop(&mut self) {
        self.map.clear()
    }
}