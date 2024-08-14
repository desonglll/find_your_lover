use std::io;

pub struct Attribute {
    pub appearance: u32,
    pub personality: u32,
    pub financial_stability: u32,
    pub sense_of_humor: u32,
    pub common_interests: u32,
    pub family_background: u32,
    pub health: u32,
    pub lifestyle: u32,
    pub career: u32,
    pub education: u32,
}
impl Attribute {
    pub fn input_attributes() -> Attribute {
        println!("请输入相亲对象的各项评分（0-100）：");

        let appearance = read_input("外貌");
        let personality = read_input("性格");
        let financial_stability = read_input("经济能力");
        let sense_of_humor = read_input("幽默感");
        let common_interests = read_input("兴趣爱好");
        let family_background = read_input("家庭背景");
        let health = read_input("健康状况");
        let lifestyle = read_input("生活习惯");
        let career = read_input("职业");
        let education = read_input("教育背景");

        Attribute {
            appearance,
            personality,
            financial_stability,
            sense_of_humor,
            common_interests,
            family_background,
            health,
            lifestyle,
            career,
            education,
        }
    }
}
pub struct Candidate {
    pub name: String,
    pub age: u8,
    pub attributes: Attribute,
}

impl Candidate {
    pub fn new(name: String, age: u8, attributes: Attribute) -> Self {
        Self {
            name,
            age,
            attributes,
        }
    }

    pub fn calculate_score(&self) -> f64 {
        let weights = [
            0.15, // appearance
            0.20, // personality
            0.15, // financial stability
            0.10, // sense of humor
            0.10, // common interests
            0.10, // family background
            0.05, // health
            0.05, // lifestyle
            0.05, // career
            0.05, // education
        ];

        let attributes = [
            self.attributes.appearance,
            self.attributes.personality,
            self.attributes.financial_stability,
            self.attributes.sense_of_humor,
            self.attributes.common_interests,
            self.attributes.family_background,
            self.attributes.health,
            self.attributes.lifestyle,
            self.attributes.career,
            self.attributes.education,
        ];

        let total_score: f64 = attributes
            .iter()
            .zip(weights.iter())
            .map(|(&attribute, &weight)| attribute as f64 * weight)
            .sum();

        total_score
    }
}
fn read_input(prompt: &str) -> u32 {
    println!("{}：", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().parse().expect("请输入有效的数字")
}
