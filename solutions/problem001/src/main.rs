pub fn translate(input: &str, offset: u8) -> String {
    let result = input.as_bytes().into_iter().map(|ch| {
        let shifted = ch + offset;

        if shifted > b'z' {
            return shifted - 26;
        }

        match shifted {
            b'a'..=b'z' => shifted,
            _ => *ch,
        }
    });
    let mut output = String::with_capacity(input.len());
    for i in result {
        output.push(i as char);
    }

    output
}

fn main() {
    let offset = b'm' - b'k';
    let input = String::from(
        "g fmnc wms bgblr rpylqjyrc gr zw fylb. rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq pcamkkclbcb. lmu ynnjw ml rfc spj."
    );
    println!("{}", translate(&input, offset));
    println!("{}", translate("map", offset));
}
