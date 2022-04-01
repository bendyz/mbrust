
extern crate bytesize;
use bytesize::ByteSize;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::fs::File;




fn main() -> io::Result<()> {
    let  f = File::open("/dev/nvme1n1");
    // let mut f = File::open("/dev/sda")?;

    let mut f = match f{
        Ok(file) => file,
        Err(_error) => panic!("Problem reading the disk: {:?}", _error)
    };
    
    let mut buffer = [0; 1024];

    let fres = f.read( &mut buffer);

    match fres{
        Ok(file) => file,
        Err(_error) => panic!("Problem reading the file: {:?}", _error)
    };

    //println!("mbr last: {:x}, {:x}",buffer[510],buffer[511]);
    let gpt = std::str::from_utf8(&buffer[512..520]);

    let gpt = match gpt{
        Ok(str) => str,
        Err(_error) => "NO GPT"
    };

    println!("IS GPT? : {}", gpt);

    //assert!(std::str::from_utf8(&buffer[512..520]).is_err());


    // and more! See the other methods for more details.

    let slice: &[u8]  = &buffer[446..510];

    // for x in 0..64{
    //     println!("slice {}: {}",x,slice[x]);
    // }


    for n in 0..3{
        println!("\nPartycja {}:", n+1);
        partycja(&slice[n*16..(n+1)*16]);
        let mbpart1 = MBRpart::new(&slice[n*16..(n+1)*16]);
        println!("info o partycji ze struct: {:?}", mbpart1.to_tuple());
    }

    // println!("\nPartycja 1: ");
    // partycja(&slice[0..16]);
    // let mbpart1 = MBRpart::new(&slice[0..16]);
    // println!("info o partycji ze struct: {:?}", mbpart1.to_tuple());

    // println!("\nPartycja 2: ");
    // partycja(&slice[16..32]);

    // println!("\nPartycja 3: ");
    // partycja(&slice[32..48]);

    // println!("\nPartycja 4: ");
    // partycja(&slice[48..64]);



    Ok(())
}


struct MBRpart{
    first_sector : u32,
    sector_count : u32,
    size : ByteSize,
    partition_type : u8,
    partition_type_string: String 
}


impl MBRpart {
    fn new(arr: &[u8]) -> MBRpart{
        let pierwszy_sektor_tab: [u8; 4] = [arr[11],arr[10],arr[9],arr[8]];
        let iloscsektorow_tab: [u8; 4] = [arr[15],arr[14],arr[13],arr[12]];
        let tsector_count= u32::from_be_bytes(iloscsektorow_tab.try_into().unwrap());
        let tsize = ByteSize::b((tsector_count) as u64 *512);
        let tpartition_type = arr[4];
        

        MBRpart{
            first_sector: u32::from_be_bytes(pierwszy_sektor_tab.try_into().unwrap()),
            sector_count: tsector_count,
            size: tsize,
            partition_type: tpartition_type,
            partition_type_string: get_partition_type_string(tpartition_type).to_string()
        }
    }

    fn to_tuple(&self) ->(String, String){
        (self.size.to_string(),self.first_sector.to_string())
    }


    

}


fn get_partition_type_string(ptype: u8) -> String{
    let mut partition_types: HashMap<u8, &str> = HashMap::new();
    partition_types.insert(0x01, "FAT12");
    partition_types.insert(0x82, "Linux swap / Solaris");
    partition_types.insert(0x83, "Linux");  
    partition_types.insert(0x0b, "W95 FAT32");
    partition_types.insert(0x0c, "W95 FAT32 (LBA)");  
    partition_types.insert(0xee, "EFI GPT");  
    partition_types.insert(0xef, "EFI (FAT-12/16/32)");  

    let mut res: String = "UNKNOWN".to_string();
    for x in partition_types{
        if x.0 == ptype{
            res = x.1.to_string();
        } 
    }

    res

}


fn partycja(arr: &[u8]){


    let mut partition_types: HashMap<u8, &str> = HashMap::new();
    partition_types.insert(0x01, "FAT12");
    partition_types.insert(0x82, "Linux swap / Solaris");
    partition_types.insert(0x83, "Linux");  
    partition_types.insert(0x0b, "W95 FAT32");
    partition_types.insert(0x0c, "W95 FAT32 (LBA)");  
    partition_types.insert(0xee, "EFI GPT");  
    partition_types.insert(0xef, "EFI (FAT-12/16/32)");  
    

    if arr[0] == 0x80 { println!("Partycja rozruchowa")};

    for x in partition_types{
        if x.0 == arr[4]{
            println!("Typ partycji to {}",x.1 );
        } 
    }

    //let iloscsektorow: i64 = (arr[14]*arr[15]) as i64;



    let pierwszy_sektor_tab: [u8; 4] = [arr[11],arr[10],arr[9],arr[8]];
    let pierwszy_sektor = u32::from_be_bytes(pierwszy_sektor_tab.try_into().unwrap());

    println!("pierwszy sektor: {}",pierwszy_sektor);

    let iloscsektorow_tab: [u8; 4] = [arr[15],arr[14],arr[13],arr[12]];
    let iloscsektorow = u32::from_be_bytes(iloscsektorow_tab.try_into().unwrap());

    println!("Ilośc sektorów: {}",iloscsektorow);

    let iloscbyteow = ByteSize::b((iloscsektorow) as u64 *512);
    println!("rozmiar partycji: {}", iloscbyteow);



}