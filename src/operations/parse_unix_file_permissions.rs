/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of the Parse UNIX file permissions operation.
 * -----------------------------------------------------------------------------
 */

use regex::Regex;

use crate::operation::{ArgSchema, ArgValue, DataType, Operation, OperationError};

/// Parse UNIX file permissions operation
pub struct ParseUNIXFilePermissions;

#[derive(Default)]
struct Perms {
    d: bool,  // directory
    sl: bool, // symbolic link
    np: bool, // named pipe
    s: bool,  // socket
    cd: bool, // character device
    bd: bool, // block device
    dr: bool, // door
    sb: bool, // sticky bit
    su: bool, // setuid
    sg: bool, // setgid
    ru: bool, // read user
    wu: bool, // write user
    eu: bool, // execute user
    rg: bool, // read group
    wg: bool, // write group
    eg: bool, // execute group
    ro: bool, // read other
    wo: bool, // write other
    eo: bool, // execute other
}

impl Operation for ParseUNIXFilePermissions {
    fn name(&self) -> &'static str {
        "Parse UNIX file permissions"
    }

    fn module(&self) -> &'static str {
        "Default"
    }

    fn description(&self) -> &'static str {
        "Given a UNIX/Linux file permission string in octal or textual format, this operation explains which permissions are granted to which user groups.<br><br>Input should be in either octal (e.g. <code>755</code>) or textual (e.g. <code>drwxr-xr-x</code>) format."
    }

    fn args_schema(&self) -> &'static [ArgSchema] {
        &[]
    }

    fn input_type(&self) -> DataType {
        DataType::String
    }

    fn output_type(&self) -> DataType {
        DataType::String
    }

    fn run(&self, input: Vec<u8>, _args: &[ArgValue]) -> Result<Vec<u8>, OperationError> {
        let input_str = String::from_utf8_lossy(&input);
        let input_str = input_str.trim();

        let mut perms = Perms::default();
        let mut textual_input = None;

        let octal_re = Regex::new(r"^[0-7]{1,4}$").unwrap();
        let textual_re = Regex::new(r"^[dlpcbDrwxsStT-]{1,10}$").unwrap();

        if octal_re.is_match(input_str) {
            let octal = input_str;
            let mut d = 0;
            let mut u = 0;
            let mut g = 0;
            let mut o = 0;

            if octal.len() == 4 {
                d = u32::from_str_radix(&octal[0..1], 8).unwrap_or(0);
                u = u32::from_str_radix(&octal[1..2], 8).unwrap_or(0);
                g = u32::from_str_radix(&octal[2..3], 8).unwrap_or(0);
                o = u32::from_str_radix(&octal[3..4], 8).unwrap_or(0);
            } else {
                if octal.len() > 0 {
                    u = u32::from_str_radix(&octal[0..1], 8).unwrap_or(0);
                }
                if octal.len() > 1 {
                    g = u32::from_str_radix(&octal[1..2], 8).unwrap_or(0);
                }
                if octal.len() > 2 {
                    o = u32::from_str_radix(&octal[2..3], 8).unwrap_or(0);
                }
            }

            perms.su = (d >> 2 & 0x1) != 0;
            perms.sg = (d >> 1 & 0x1) != 0;
            perms.sb = (d & 0x1) != 0;

            perms.ru = (u >> 2 & 0x1) != 0;
            perms.wu = (u >> 1 & 0x1) != 0;
            perms.eu = (u & 0x1) != 0;

            perms.rg = (g >> 2 & 0x1) != 0;
            perms.wg = (g >> 1 & 0x1) != 0;
            perms.eg = (g & 0x1) != 0;

            perms.ro = (o >> 2 & 0x1) != 0;
            perms.wo = (o >> 1 & 0x1) != 0;
            perms.eo = (o & 0x1) != 0;
        } else if textual_re.is_match(input_str) {
            textual_input = Some(input_str.to_string());
            let textual = input_str.as_bytes();

            if textual.len() > 0 {
                match textual[0] as char {
                    'd' => perms.d = true,
                    'l' => perms.sl = true,
                    'p' => perms.np = true,
                    's' => perms.s = true,
                    'c' => perms.cd = true,
                    'b' => perms.bd = true,
                    'D' => perms.dr = true,
                    _ => {}
                }
            }

            if textual.len() > 1 {
                perms.ru = textual[1] == b'r';
            }
            if textual.len() > 2 {
                perms.wu = textual[2] == b'w';
            }
            if textual.len() > 3 {
                match textual[3] as char {
                    'x' => perms.eu = true,
                    's' => {
                        perms.eu = true;
                        perms.su = true;
                    }
                    'S' => perms.su = true,
                    _ => {}
                }
            }

            if textual.len() > 4 {
                perms.rg = textual[4] == b'r';
            }
            if textual.len() > 5 {
                perms.wg = textual[5] == b'w';
            }
            if textual.len() > 6 {
                match textual[6] as char {
                    'x' => perms.eg = true,
                    's' => {
                        perms.eg = true;
                        perms.sg = true;
                    }
                    'S' => perms.sg = true,
                    _ => {}
                }
            }

            if textual.len() > 7 {
                perms.ro = textual[7] == b'r';
            }
            if textual.len() > 8 {
                perms.wo = textual[8] == b'w';
            }
            if textual.len() > 9 {
                match textual[9] as char {
                    'x' => perms.eo = true,
                    't' => {
                        perms.eo = true;
                        perms.sb = true;
                    }
                    'T' => perms.sb = true,
                    _ => {}
                }
            }
        } else {
            return Err(OperationError::InvalidInput("Invalid input format.\nPlease enter the permissions in either octal (e.g. 755) or textual (e.g. drwxr-xr-x) format.".to_string()));
        }

        let mut output = String::new();
        output += &format!("Textual representation: {}\n", perms_to_str(&perms));
        output += &format!("Octal representation:   {}", perms_to_octal(&perms));

        if textual_input.is_some() {
            output += &format!("\nFile type: {}", ft_from_perms(&perms));
        }

        if perms.su {
            output += "\nThe setuid flag is set";
        }
        if perms.sg {
            output += "\nThe setgid flag is set";
        }
        if perms.sb {
            output += "\nThe sticky bit is set";
        }

        output += "\n\n +---------+-------+-------+-------+\n";
        output += " |         | User  | Group | Other |\n";
        output += " +---------+-------+-------+-------+\n";
        output += &format!(
            " |    Read |   {}   |   {}   |   {}   |\n",
            if perms.ru { "X" } else { " " },
            if perms.rg { "X" } else { " " },
            if perms.ro { "X" } else { " " }
        );
        output += " +---------+-------+-------+-------+\n";
        output += &format!(
            " |   Write |   {}   |   {}   |   {}   |\n",
            if perms.wu { "X" } else { " " },
            if perms.wg { "X" } else { " " },
            if perms.wo { "X" } else { " " }
        );
        output += " +---------+-------+-------+-------+\n";
        output += &format!(
            " | Execute |   {}   |   {}   |   {}   |\n",
            if perms.eu { "X" } else { " " },
            if perms.eg { "X" } else { " " },
            if perms.eo { "X" } else { " " }
        );
        output += " +---------+-------+-------+-------+";

        Ok(output.into_bytes())
    }
}

fn perms_to_str(perms: &Perms) -> String {
    let mut s = String::new();
    let mut ftype = "-";
    if perms.d {
        ftype = "d";
    } else if perms.sl {
        ftype = "l";
    } else if perms.np {
        ftype = "p";
    } else if perms.s {
        ftype = "s";
    } else if perms.cd {
        ftype = "c";
    } else if perms.bd {
        ftype = "b";
    } else if perms.dr {
        ftype = "D";
    }
    s.push_str(ftype);

    s.push(if perms.ru { 'r' } else { '-' });
    s.push(if perms.wu { 'w' } else { '-' });
    if perms.eu && perms.su {
        s.push('s');
    } else if perms.su {
        s.push('S');
    } else if perms.eu {
        s.push('x');
    } else {
        s.push('-');
    }

    s.push(if perms.rg { 'r' } else { '-' });
    s.push(if perms.wg { 'w' } else { '-' });
    if perms.eg && perms.sg {
        s.push('s');
    } else if perms.sg {
        s.push('S');
    } else if perms.eg {
        s.push('x');
    } else {
        s.push('-');
    }

    s.push(if perms.ro { 'r' } else { '-' });
    s.push(if perms.wo { 'w' } else { '-' });
    if perms.eo && perms.sb {
        s.push('t');
    } else if perms.sb {
        s.push('T');
    } else if perms.eo {
        s.push('x');
    } else {
        s.push('-');
    }

    s
}

fn perms_to_octal(perms: &Perms) -> String {
    let mut d = 0;
    let mut u = 0;
    let mut g = 0;
    let mut o = 0;

    if perms.su {
        d += 4;
    }
    if perms.sg {
        d += 2;
    }
    if perms.sb {
        d += 1;
    }

    if perms.ru {
        u += 4;
    }
    if perms.wu {
        u += 2;
    }
    if perms.eu {
        u += 1;
    }

    if perms.rg {
        g += 4;
    }
    if perms.wg {
        g += 2;
    }
    if perms.eg {
        g += 1;
    }

    if perms.ro {
        o += 4;
    }
    if perms.wo {
        o += 2;
    }
    if perms.eo {
        o += 1;
    }

    format!("{}{}{}{}", d, u, g, o)
}

fn ft_from_perms(perms: &Perms) -> &'static str {
    if perms.d {
        "Directory"
    } else if perms.sl {
        "Symbolic link"
    } else if perms.np {
        "Named pipe"
    } else if perms.s {
        "Socket"
    } else if perms.cd {
        "Character device"
    } else if perms.bd {
        "Block device"
    } else if perms.dr {
        "Door"
    } else {
        "Regular file"
    }
}
