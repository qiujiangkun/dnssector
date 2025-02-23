use dnssector;

mod tests {
    use super::dnssector::*;

    #[test]
    fn test_empty_packet() {
        let data: Vec<u8> = vec![];
        let dns_sector = DNSSector::new(data).unwrap();
        assert!(dns_sector.parse().is_err());
    }

    #[test]
    fn test_packet_too_small() {
        let data_small: Vec<u8> = vec![1; 11];
        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());

        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::PacketTooSmall => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_packet_has_two_questions() {
        let data_small: Vec<u8> = vec![0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0];
        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::InvalidPacket(_) => {}
            a => panic!("type: {:?}", a),
        }
    }

    #[test]
    fn test_packet_advertises_one_question_but_is_missing_section() {
        let data_small: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::InternalError(_) => {}
            a => panic!("type: {:?}", a),
        }
    }

    #[test]
    fn test_packet_has_empty_name() {
        let data_small: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_ok());
    }

    #[test]
    fn test_packet_name_does_not_end() {
        let data_small: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, b'a', 0, 0, 0, 1];
        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::PacketTooSmall => {}
            a => panic!("type: {:?}", a),
        }
    }

    #[test]
    fn test_packet_label_too_long() {
        let data_small: Vec<u8> = vec![
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 64, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 0, 0, 0, 0, 1,
        ];
        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::InvalidName(_) => {}
            a => panic!("type: {:?}", a),
        }
    }

    #[test]
    fn test_packet_name_too_long() {
        let data_small: Vec<u8> = vec![
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 0, 0, 0, 0, 1,
        ];
        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::InvalidName(_) => {}
            a => panic!("type: {:?}", a),
        }
    }

    #[test]
    fn test_packet_name_too_long_with_compression() {
        let mut data_small: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 0, 3, 0, 0, 0, 0];

        // query
        data_small.extend(vec![
            63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 0, 0,
            1, 0, 1,
        ]);

        // 1st answer
        data_small.extend(vec![
            63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 0xc0,
            0x0c, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 1, 2, 3, 4,
        ]);

        // 2nd answer
        data_small.extend(vec![
            63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 0xc0,
            0x51, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 1, 2, 3, 4,
        ]);

        // 3rd answer
        data_small.extend(vec![
            63, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97,
            97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 97, 0xc0,
            0x9d, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 1, 2, 3, 4,
        ]);

        let dns_sector = DNSSector::new(data_small).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::InvalidName("Label length too long") => {}
            DSError::InvalidPacket("A question shouldn\'t also contain answers") => {}
            a => panic!("type: {:?}", a),
        }
    }

    #[test]
    fn test_packet_bogus_ipv6_length() {
        let mut data: Vec<u8> = vec![0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0];

        // query
        data.extend(vec![0, 0, 28, 0, 1]);

        // 1st answer
        data.extend(vec![0, 0, 28, 0, 1, 0, 0, 0, 0, 0, 4, 1, 2, 3, 4]);

        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::InvalidPacket("AAAA record doesn\'t include a 16 bytes IP address") => {}
            DSError::InvalidPacket("A question shouldn\'t also contain answers") => {}
            a => panic!("type: {:?}", a),
        }
    }

    #[test]
    fn test_packet_valid_response_1() {
        let data = vec![
            38, 44, 129, 160, 0, 1, 0, 2, 0, 0, 0, 1, 3, 99, 57, 120, 3, 111, 114, 71, 0, 0, 1, 0,
            1, 192, 12, 0, 1, 0, 1, 0, 0, 167, 29, 0, 4, 78, 194, 219, 1, 192, 12, 0, 46, 0, 1, 0,
            0, 167, 29, 0, 91, 0, 1, 13, 2, 0, 0, 168, 192, 89, 56, 71, 147, 89, 16, 186, 147, 82,
            60, 3, 99, 57, 120, 3, 111, 114, 103, 0, 153, 235, 139, 49, 43, 255, 159, 252, 196,
            189, 29, 77, 88, 132, 233, 31, 133, 88, 104, 42, 139, 12, 101, 158, 121, 95, 105, 180,
            59, 216, 202, 174, 113, 201, 121, 23, 4, 26, 241, 134, 233, 52, 104, 120, 80, 237, 252,
            215, 146, 44, 120, 229, 63, 16, 95, 19, 209, 103, 165, 196, 195, 151, 222, 52, 0, 0,
            41, 2, 0, 0, 0, 128, 0, 0, 0,
        ];
        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse().expect("Valid packet couldn't be parsed");
        let flags = ret.flags();
        assert_eq!(flags & DNS_FLAG_DO, DNS_FLAG_DO);
        assert_eq!(flags & DNS_FLAG_QR, DNS_FLAG_QR);
        assert_eq!(flags & DNS_FLAG_AD, DNS_FLAG_AD);
        assert_eq!(flags & DNS_FLAG_RD, DNS_FLAG_RD);
        assert_eq!(flags, 0x800081a0);
    }

    #[test]
    fn test_packet_valid_dname_response_2() {
        let data = vec![
            0x4d, 0x32, 0x85, 0x00, 0x00, 0x01, 0x00, 0x05, 0x00, 0x05, 0x00, 0x01, 0x01, 0x61,
            0x05, 0x61, 0x6c, 0x69, 0x61, 0x73, 0x04, 0x73, 0x74, 0x64, 0x63, 0x03, 0x6f, 0x72,
            0x67, 0x00, 0x00, 0x01, 0x00, 0x01, 0xc0, 0x0e, 0x00, 0x27, 0x00, 0x01, 0x00, 0x00,
            0x0e, 0x10, 0x00, 0x0e, 0x08, 0x70, 0x75, 0x72, 0x65, 0x66, 0x74, 0x70, 0x64, 0x03,
            0x6f, 0x72, 0x67, 0x00, 0xc0, 0x0e, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x00, 0x0e, 0x10,
            0x00, 0x5c, 0x00, 0x27, 0x0d, 0x03, 0x00, 0x00, 0x0e, 0x10, 0x59, 0x53, 0xb6, 0xf8,
            0x59, 0x2c, 0x29, 0xf8, 0x2a, 0xff, 0x04, 0x73, 0x74, 0x64, 0x63, 0x03, 0x6f, 0x72,
            0x67, 0x00, 0x0c, 0xb4, 0xe8, 0xb4, 0xa2, 0x1c, 0xff, 0xd5, 0xfe, 0x94, 0x26, 0xf7,
            0xc8, 0x80, 0x43, 0x33, 0xce, 0xaf, 0x50, 0x57, 0xf3, 0xa0, 0x52, 0xb7, 0xf8, 0x70,
            0xd1, 0xcf, 0x21, 0xe2, 0x92, 0x04, 0xcf, 0x10, 0x6d, 0xc1, 0xcb, 0xfa, 0x13, 0xdb,
            0x9e, 0x9c, 0xa5, 0xc2, 0xcd, 0x9f, 0x05, 0xc4, 0xf0, 0x82, 0xfb, 0x75, 0x30, 0xce,
            0x6f, 0x06, 0x33, 0xfb, 0x34, 0x00, 0xc2, 0x7d, 0xf6, 0x48, 0xc0, 0x0c, 0x00, 0x05,
            0x00, 0x01, 0x00, 0x00, 0x0e, 0x10, 0x00, 0x0d, 0x01, 0x61, 0x08, 0x70, 0x75, 0x72,
            0x65, 0x66, 0x74, 0x70, 0x64, 0xc0, 0x19, 0xc0, 0xb0, 0x00, 0x01, 0x00, 0x01, 0x00,
            0x00, 0xa8, 0xc0, 0x00, 0x04, 0x4e, 0xc2, 0xdb, 0x01, 0xc0, 0xb0, 0x00, 0x2e, 0x00,
            0x01, 0x00, 0x00, 0xa8, 0xc0, 0x00, 0x60, 0x00, 0x01, 0x0d, 0x02, 0x00, 0x00, 0xa8,
            0xc0, 0x59, 0x53, 0xb6, 0xf2, 0x59, 0x2c, 0x29, 0xf2, 0x31, 0xf3, 0x08, 0x70, 0x75,
            0x72, 0x65, 0x66, 0x74, 0x70, 0x64, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x42, 0x6e, 0xcd,
            0xad, 0xea, 0xca, 0xac, 0xdf, 0x7c, 0xd0, 0x81, 0xd7, 0x0a, 0x3e, 0xad, 0x83, 0xcf,
            0xd6, 0x76, 0x05, 0x28, 0xbf, 0x46, 0xe1, 0x24, 0xb7, 0x47, 0x54, 0x85, 0x48, 0xd1,
            0x4e, 0x10, 0x63, 0xe3, 0x0c, 0xb6, 0xa4, 0xb9, 0xb9, 0x81, 0x03, 0x84, 0xd1, 0x46,
            0x38, 0xf7, 0xeb, 0x86, 0x5d, 0x9d, 0x45, 0x2d, 0xda, 0x3a, 0xeb, 0xea, 0xf0, 0xf9,
            0x28, 0xdf, 0x81, 0x6c, 0xf7, 0x20, 0x62, 0x6c, 0x37, 0x71, 0x64, 0x66, 0x38, 0x6e,
            0x31, 0x72, 0x69, 0x68, 0x73, 0x37, 0x65, 0x6c, 0x35, 0x64, 0x35, 0x36, 0x63, 0x6b,
            0x61, 0x76, 0x33, 0x72, 0x71, 0x68, 0x65, 0x63, 0x31, 0x68, 0xc0, 0xb2, 0x00, 0x32,
            0x00, 0x01, 0x00, 0x00, 0x0e, 0x10, 0x00, 0x23, 0x01, 0x00, 0x00, 0x02, 0x01, 0x01,
            0x14, 0x93, 0x19, 0xc4, 0xdd, 0xd4, 0x84, 0x6b, 0x44, 0x03, 0x82, 0x82, 0x8b, 0xca,
            0x77, 0xdf, 0x26, 0x93, 0xee, 0xe0, 0x20, 0x00, 0x06, 0x40, 0x00, 0x00, 0x00, 0x00,
            0x02, 0xc1, 0x39, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x00, 0x0e, 0x10, 0x00, 0x60, 0x00,
            0x32, 0x0d, 0x03, 0x00, 0x00, 0x0e, 0x10, 0x59, 0x53, 0xb6, 0xf2, 0x59, 0x2c, 0x29,
            0xf2, 0x31, 0xf3, 0x08, 0x70, 0x75, 0x72, 0x65, 0x66, 0x74, 0x70, 0x64, 0x03, 0x6f,
            0x72, 0x67, 0x00, 0x22, 0x82, 0x60, 0xfb, 0x06, 0xea, 0xc4, 0xd0, 0xdd, 0x59, 0xd3,
            0x9a, 0x9b, 0x43, 0xd9, 0xc5, 0xb3, 0xab, 0x86, 0x1e, 0xaf, 0xc3, 0xb9, 0x90, 0x0f,
            0xc1, 0xd5, 0x40, 0x14, 0x1e, 0xb3, 0x12, 0x4d, 0xdf, 0x73, 0x1a, 0x5f, 0xbe, 0x99,
            0x4d, 0x6f, 0xd7, 0xf5, 0xbf, 0x9b, 0xed, 0x63, 0x7e, 0x6c, 0xe9, 0x2a, 0x74, 0xc3,
            0xd4, 0x8d, 0x5e, 0x98, 0x72, 0xfc, 0xc1, 0x30, 0xdf, 0x68, 0x35, 0xc0, 0xb2, 0x00,
            0x02, 0x00, 0x01, 0x00, 0x01, 0x5b, 0xd2, 0x00, 0x0e, 0x04, 0x66, 0x72, 0x65, 0x65,
            0x02, 0x6e, 0x73, 0x03, 0x63, 0x39, 0x78, 0xc0, 0x19, 0xc0, 0xb2, 0x00, 0x02, 0x00,
            0x01, 0x00, 0x01, 0x5b, 0xd2, 0x00, 0x0a, 0x07, 0x72, 0x65, 0x63, 0x69, 0x74, 0x61,
            0x6c, 0xc2, 0x06, 0xc0, 0xb2, 0x00, 0x2e, 0x00, 0x01, 0x00, 0x01, 0x5b, 0xd2, 0x00,
            0x60, 0x00, 0x02, 0x0d, 0x02, 0x00, 0x01, 0x5b, 0xd2, 0x59, 0x53, 0xb6, 0xf2, 0x59,
            0x2c, 0x29, 0xf2, 0x31, 0xf3, 0x08, 0x70, 0x75, 0x72, 0x65, 0x66, 0x74, 0x70, 0x64,
            0x03, 0x6f, 0x72, 0x67, 0x00, 0x9e, 0xa8, 0x72, 0xd1, 0xc4, 0x14, 0x8b, 0x1e, 0x5d,
            0x85, 0xce, 0x84, 0x51, 0x2c, 0xad, 0x8a, 0xd5, 0x88, 0x86, 0x6d, 0x10, 0xa5, 0x1c,
            0x4c, 0x1b, 0x20, 0xd4, 0x13, 0xc2, 0xfd, 0xe5, 0x77, 0xca, 0xd5, 0x5d, 0x61, 0xb6,
            0x23, 0xab, 0xbb, 0xa1, 0xd7, 0x90, 0xd4, 0x4f, 0x44, 0x11, 0xfa, 0xee, 0xe2, 0x0c,
            0x26, 0xf5, 0x7f, 0x79, 0xf3, 0x3f, 0xe7, 0x0f, 0x4e, 0xe6, 0xaf, 0x11, 0x08, 0x00,
            0x00, 0x29, 0x10, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
        ];
        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse().expect("Valid packet couldn't be parsed");
        let flags = ret.flags();
        assert_eq!(flags & DNS_FLAG_DO, DNS_FLAG_DO);
        assert_eq!(flags & DNS_FLAG_QR, DNS_FLAG_QR);
        assert_eq!(flags & DNS_FLAG_AD, 0);
        assert_eq!(flags & DNS_FLAG_RD, DNS_FLAG_RD);
        assert_eq!(flags, 0x80008500);
    }

    #[test]
    fn test_packet_mx_rr() {
        let data = vec![
            0x45, 0x69, 0x85, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x03, 0x63,
            0x39, 0x78, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x00, 0x0f, 0x00, 0x01, 0xc0, 0x0c, 0x00,
            0x0f, 0x00, 0x01, 0x00, 0x00, 0xa8, 0xc0, 0x00, 0x0f, 0x00, 0x0a, 0x07, 0x72, 0x65,
            0x63, 0x69, 0x74, 0x61, 0x6c, 0x02, 0x6d, 0x78, 0xc0, 0x0c, 0xc0, 0x0c, 0x00, 0x02,
            0x00, 0x01, 0x00, 0x01, 0x5b, 0xd2, 0x00, 0x0a, 0x04, 0x66, 0x72, 0x65, 0x65, 0x02,
            0x6e, 0x73, 0xc0, 0x0c, 0xc0, 0x0c, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x5b, 0xd2,
            0x00, 0x0a, 0x07, 0x72, 0x65, 0x63, 0x69, 0x74, 0x61, 0x6c, 0xc0, 0x45, 0xc0, 0x27,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xa8, 0xc0, 0x00, 0x04, 0x25, 0x3b, 0xee, 0xd5,
            0xc0, 0x40, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xa8, 0xc0, 0x00, 0x04, 0x4e, 0xc2,
            0xdb, 0x01, 0xc0, 0x56, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xa8, 0xc0, 0x00, 0x04,
            0x25, 0x3b, 0xee, 0xd5,
        ];
        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse().expect("Valid packet couldn't be parsed");
        let flags = ret.flags();
        assert_eq!(flags & DNS_FLAG_DO, 0);
        assert_eq!(flags & DNS_FLAG_QR, DNS_FLAG_QR);
        assert_eq!(flags & DNS_FLAG_AD, 0);
        assert_eq!(flags & DNS_FLAG_RD, DNS_FLAG_RD);
        assert_eq!(flags, 0x8500);
    }

    #[test]
    fn test_packet_soa_rr() {
        let data = vec![
            0x57, 0xfe, 0x85, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0x00, 0x02, 0x03, 0x63,
            0x39, 0x78, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x00, 0x06, 0x00, 0x01, 0xc0, 0x0c, 0x00,
            0x06, 0x00, 0x01, 0x00, 0x01, 0x51, 0x80, 0x00, 0x2b, 0x04, 0x66, 0x72, 0x65, 0x65,
            0x02, 0x6e, 0x73, 0xc0, 0x0c, 0x0a, 0x68, 0x6f, 0x73, 0x74, 0x6d, 0x61, 0x73, 0x74,
            0x65, 0x72, 0xc0, 0x0c, 0x59, 0x2c, 0x8c, 0x80, 0x00, 0x00, 0x70, 0x80, 0x00, 0x00,
            0x1c, 0x20, 0x00, 0x09, 0x3a, 0x80, 0x00, 0x00, 0x0e, 0x10, 0xc0, 0x0c, 0x00, 0x02,
            0x00, 0x01, 0x00, 0x01, 0x5b, 0xd2, 0x00, 0x02, 0xc0, 0x25, 0xc0, 0x0c, 0x00, 0x02,
            0x00, 0x01, 0x00, 0x01, 0x5b, 0xd2, 0x00, 0x0a, 0x07, 0x72, 0x65, 0x63, 0x69, 0x74,
            0x61, 0x6c, 0xc0, 0x2a, 0xc0, 0x25, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xa8, 0xc0,
            0x00, 0x04, 0x4e, 0xc2, 0xdb, 0x01, 0xc0, 0x6a, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
            0xa8, 0xc0, 0x00, 0x04, 0x25, 0x3b, 0xee, 0xd5,
        ];
        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse().expect("Valid packet couldn't be parsed");
        let flags = ret.flags();
        assert_eq!(flags & DNS_FLAG_DO, 0);
        assert_eq!(flags & DNS_FLAG_QR, DNS_FLAG_QR);
        assert_eq!(flags & DNS_FLAG_AD, 0);
        assert_eq!(flags & DNS_FLAG_RD, DNS_FLAG_RD);
        assert_eq!(flags, 0x8500);
    }

    #[test]
    fn test_packet_edns_client_subnet_1() {
        let data = vec![
            0x43, 0x96, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03, 0x63,
            0x39, 0x78, 0x03, 0x6f, 0x72, 0x67, 0x00, 0x00, 0x10, 0x00, 0x01, 0x00, 0x00, 0x29,
            0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x00, 0x08, 0x00, 0x07, 0x00, 0x01,
            0x18, 0x00, 0xa3, 0x05, 0x01,
        ];
        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse().expect("Valid packet couldn't be parsed");
        let flags = ret.flags();
        assert_eq!(flags & DNS_FLAG_DO, 0);
        assert_eq!(flags & DNS_FLAG_QR, 0);
        assert_eq!(flags & DNS_FLAG_AD, 0);
        assert_eq!(flags & DNS_FLAG_RD, DNS_FLAG_RD);
        assert_eq!(flags, 0x100);
    }

    #[test]
    fn test_packet_custom_edns_record() {
        let data = vec![
            0x25, 0x89, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x05, 0x64,
            0x65, 0x62, 0x75, 0x67, 0x07, 0x6f, 0x70, 0x65, 0x6e, 0x64, 0x6e, 0x73, 0x03, 0x63,
            0x6f, 0x6d, 0x00, 0x00, 0x10, 0x00, 0x01, 0x00, 0x00, 0x29, 0x10, 0x00, 0x00, 0x00,
            0x80, 0x00, 0x00, 0x13, 0x00, 0x04, 0x00, 0x0f, 0x4f, 0x70, 0x65, 0x6e, 0x44, 0x4e,
            0x53, 0xca, 0xfe, 0xba, 0xbe, 0xde, 0xad, 0xbe, 0xef,
        ];
        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse().expect("Valid packet couldn't be parsed");
        let flags = ret.flags();
        assert_eq!(flags & DNS_FLAG_DO, DNS_FLAG_DO);
        assert_eq!(flags & DNS_FLAG_QR, 0);
        assert_eq!(flags & DNS_FLAG_AD, 0);
        assert_eq!(flags & DNS_FLAG_RD, DNS_FLAG_RD);
        assert_eq!(flags, 0x80000100);
    }

    #[test]
    fn test_packet_with_no_question() {
        let data = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 4, 4, 4, 4, 63, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 59, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 41, 0, 0, 0, 0, 0, 10, 0, 0,
        ];
        let dns_sector = DNSSector::new(data).unwrap();
        let ret = dns_sector.parse();
        assert!(ret.is_err());
        match ret.err().expect("error").downcast::<DSError>().unwrap() {
            DSError::InvalidPacket(_) => {}
            _ => panic!(),
        }
    }
}
