use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

/// Tupper's self-referential formula variables
pub struct Tuppers {
    pub k: BigUint,
    pub x: u32,
    pub y: u32
}

/// Technically, your name or basically anything imaginable is also one of them constants,
/// just plot them and add 17 with the bitmap to find out!
/// Read: https://en.wikipedia.org/wiki/Tupper%27s_self-referential_formula#Plots
pub enum Constant {
    Tuppers,
    Pacman,
    Euler
}

/// k constants (k to k + 17)
/// Plot of Tupper's self-referential formula itself
pub const K: &[u8] = b"4858450636189713423582095962494202044581400587983244549483093085061934704708809928450644769865524364849997247024915119110411605739177407856919754326571855442057210445735883681829823754139634338225199452191651284348332905131193199953502413758765239264874613394906870130562295813219481113685339535565290850023875092856892694555974281546386510730049106723058933586052544096664351265349363643957125565695936815184334857605266940161251266951421550539554519153785457525756590740540157929001765967965480064427829131488548259914721248506352686630476300";

/// Plot of Pacmans
pub const PACMAN: &[u8] = b"144520248970897582847942537337194567481277782215150702479718813968549088735682987348888251320905766438178883231976923440016667764749242125128995265907053708020473915320841631792025549005418004768657201699730466383394901601374319715520996181145249781945019068359500510657804325640801197867556863142280259694206254096081665642417367403946384170774537427319606443899923010379398938675025786929455234476319291860957618345432248004921728033349419816206749854472038193939738513848960476759782673313437697051994580681869819330446336774047268864";

/// Plot of the Euler's identity
pub const EULER: &[u8] = b"2352035939949658122140829649197960929306974813625028263292934781954073595495544614140648457342461564887325223455620804204796011434955111022376601635853210476633318991990462192687999109308209472315419713652238185967518731354596984676698288025582563654632501009155760415054499960";

impl Tuppers {
    /// parse k constant to a BigNumber / BigUint.
    pub fn bignumber_k(k: &Constant) -> BigUint {
        match k {
            Constant::Tuppers => return BigUint::parse_bytes(K, 10).unwrap(),
            Constant::Pacman  => return BigUint::parse_bytes(PACMAN, 10).unwrap(),
            Constant::Euler   => return BigUint::parse_bytes(EULER, 10).unwrap()
        }
    }

    ///
    /// Tupper's self-referential formula
    /// 
    /// ---------------------------------------------------------------------------------
    /// | 1/2 < floor(mod(floor(y / 17) * 2 ^ (-17 * floor(x) - mod(floor(y), 17)), 2)) |
    /// ---------------------------------------------------------------------------------
    /// 
    /// https://en.wikipedia.org/wiki/Tupper%27s_self-referential_formula
    /// 

    /// self-referential formula depicts if or not to plot the pixel on the bitmap.
    #[inline]
    pub fn _self_referential_formula(self) -> bool {
        (1_f32 / 2_f32) < (((self.k + self.y) / (17_u32) / BigUint::pow(&BigUint::from(2_u32), 17 * self.x + self.y % 17)) % (2_u32))
            .to_f32()
            .unwrap()        
    }

    /// self-referential formula depicts if or not to plot the pixel on the bitmap (but inverse).
    #[inline]
    pub fn self_referential_formula_inverse(self) -> bool {
        (
            ((self.k + self.y) / (17_u32) / BigUint::pow(&BigUint::from(2_u32), 17 * self.x + self.y % 17)) % (2_u32)
        ).is_zero()        
    }
}