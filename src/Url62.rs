use uuid::Uuid;

struct Url62 {}

impl Url62 {
    /**
	 * Encode UUID to Url62 id
	 *
	 * @param uuid UUID to be encoded
	 * @return url62 encoded UUID
	 */
    /**
	 * Encode UUID to Url62 id
	 *
	 * @param uuid UUID to be encoded
	 * @return url62 encoded UUID
	 */
    fn encode(uuid: &Uuid) -> String {
        // let pair: BigInteger = UuidConverter::to_big_integer(uuid);
        // return Base62::encode(pair);
        return uuid.to_string();
    }

    /**
	 * Decode url62 id to UUID
	 *
	 * @param id url62 encoded id
	 * @return decoded UUID
	 */
    /**
	 * Decode url62 id to UUID
	 *
	 * @param id url62 encoded id
	 * @return decoded UUID
	 */
    pub fn decode(id: &String) -> Uuid {
        // return Uuid::parse_str(id).unwrap();
        // let decoded: BigInteger = Base62::decode(&id);
        base_62::decode(&id);
        return Uuid::parse_str(id).unwrap();
        // return UuidConverter::to_uuid(decoded);
    }
}


