/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Version:     1.0.0
 * Author:      Michael Weiss
 * Source:      Ported from GCHQ's CyberChef (JavaScript)
 * License:     Apache-2.0
 * Description: Implementation of an rxchef operation.
 * -----------------------------------------------------------------------------
 */

use x509_parser::parse_x509_certificate;

pub fn test_x509() {
    let _ = parse_x509_certificate(b"");
}
