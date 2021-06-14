// This file is part of Hyperspace.
//
// Copyright (C) 2018-2021 Hyperspace Network
// SPDX-License-Identifier: GPL-3.0
//
// Hyperspace is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Hyperspace is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Hyperspace. If not, see <https://www.gnu.org/licenses/>.

// --- substrate ---
use sp_runtime::assert_eq_error_rate;
// --- hyperspace ---
use crate::{
	inflation::*,
	mock::{Balance, *},
	*,
};

#[test]
fn compute_total_payout_should_work() {
	let initial_issuance = 2_000_000_000;
	let hard_cap = 10_000_000_000;

	// year, expect inflation, expect inflation rate, payout fraction
	let inflation_spec = [
		(1_u32, 80_000_000 as Balance, 4_f64, 0_u32),
		(2, 111773288, 5.37, 35),
		(3, 134746988, 6.15, 50),
		(4, 152702246, 6.56, 77),
		(5, 167131170, 6.74, 33),
		(6, 178823310, 6.76, 81),
		(7, 188269290, 6.66, 100),
		(8, 195807997, 6.5, 50),
		(9, 201691938, 6.28, 50),
		(10, 206120193, 6.04, 50),
		(11, 209256586, 5.79, 50),
		(12, 211240394, 5.52, 50),
		(13, 212192984, 5.26, 50),
		(14, 212222107, 4.99, 50),
		(15, 211424761, 4.74, 50),
		(16, 209889164, 4.49, 50),
		(17, 207696141, 4.25, 50),
		(18, 204920129, 4.03, 50),
		(19, 201629917, 3.81, 50),
		(20, 197889214, 3.6, 50),
		(21, 193757077, 3.4, 50),
		(22, 189288266, 3.21, 50),
		(23, 184533528, 3.04, 50),
		(24, 179539840, 2.87, 50),
		(25, 174350616, 2.71, 50),
		(26, 169005896, 2.55, 50),
		(27, 163542518, 2.41, 50),
		(28, 157994277, 2.27, 50),
		(29, 152392074, 2.14, 50),
		(30, 146764063, 2.02, 50),
		(31, 141135789, 1.91, 50),
		(32, 135530328, 1.8, 50),
		(33, 129968412, 1.69, 50),
		(34, 124468569, 1.59, 50),
		(35, 119047241, 1.5, 50),
		(36, 113718914, 1.41, 50),
		(37, 108496241, 1.33, 50),
		(38, 103390154, 1.25, 50),
		(39, 98409989, 1.17, 50),
		(40, 93563589, 1.1, 50),
		(41, 88857423, 1.04, 50),
		(42, 84296681, 0.97, 50),
		(43, 79885384, 0.91, 50),
		(44, 75626477, 0.86, 50),
		(45, 71521925, 0.8, 50),
		(46, 67572798, 0.75, 50),
		(47, 63779362, 0.71, 50),
		(48, 60141154, 0.66, 50),
		(49, 56657063, 0.62, 50),
		(50, 53325399, 0.58, 50),
		(51, 50143961, 0.54, 50),
		(52, 47110102, 0.51, 50),
		(53, 44220788, 0.47, 50),
		(54, 41472651, 0.44, 50),
		(55, 38862044, 0.41, 50),
		(56, 36385085, 0.38, 50),
		(57, 34037703, 0.36, 50),
		(58, 31815678, 0.33, 50),
		(59, 29714675, 0.31, 50),
		(60, 27730280, 0.29, 50),
		(61, 25858031, 0.27, 50),
		(62, 24093441, 0.25, 50),
		(63, 22432029, 0.23, 50),
		(64, 20869334, 0.21, 50),
		(65, 19400941, 0.2, 50),
		(66, 18022494, 0.18, 50),
		(67, 16729713, 0.17, 50),
		(68, 15518405, 0.16, 50),
		(69, 14384476, 0.15, 50),
		(70, 13323940, 0.14, 50),
		(71, 12332925, 0.13, 50),
		(72, 11407683, 0.12, 50),
		(73, 10544590, 0.11, 50),
		(74, 9740152, 0.1, 50),
		(75, 8991009, 0.09, 50),
		(76, 8293933, 0.08, 50),
		(77, 7645831, 0.08, 50),
		(78, 7043743, 0.07, 50),
		(79, 6484843, 0.07, 50),
		(80, 5966438, 0.06, 50),
		(81, 5485962, 0.06, 50),
		(82, 5040980, 0.05, 50),
		(83, 4629177, 0.05, 50),
		(84, 4248362, 0.04, 50),
		(85, 3896461, 0.04, 50),
		(86, 3571514, 0.04, 50),
		(87, 3271672, 0.03, 50),
		(88, 2995190, 0.03, 50),
		(89, 2740428, 0.03, 50),
		(90, 2505842, 0.03, 50),
		(91, 2289982, 0.02, 50),
		(92, 2091488, 0.02, 50),
		(93, 1909086, 0.02, 50),
		(94, 1741584, 0.02, 50),
		(95, 1587864, 0.02, 50),
		(96, 1446887, 0.01, 50),
		(97, 1317678, 0.01, 50),
		(98, 1199332, 0.01, 50),
		(99, 1091004, 0.01, 50),
		(100, 991910, 0.01, 50),
	];
	let mut total_left: EtpBalance<Test> = hard_cap - initial_issuance;

	for &(year, exp_inflation, exp_inflation_rate, payout_fraction) in inflation_spec.iter() {
		let payout_fraction = Perbill::from_percent(payout_fraction);
		let (payout, inflation) = compute_total_payout::<Test>(
			MILLISECONDS_PER_YEAR,
			((year - 1) as TsInMs) * MILLISECONDS_PER_YEAR,
			total_left,
			payout_fraction,
		);

		// eprintln!("{}\n{}\n", inflation, exp_inflation);

		assert_eq!(payout, payout_fraction * inflation);
		assert_eq_error_rate!(inflation, exp_inflation, if inflation == 0 { 0 } else { 3 });
		assert_eq_error_rate!(
			inflation as f64 / (hard_cap - total_left) as f64,
			exp_inflation_rate / 100.00_f64,
			0.01_f64 / 100.00_f64
		);

		total_left -= inflation;
	}
}

#[test]
fn compute_dna_reward_should_work() {
	const COIN: Balance = 1_000_000_000;
	const PRECISION: f64 = 10_000.0000;

	for (month, exp_dna_reward) in (1..=36).zip(
		[
			0.0761_f64, 0.1522, 0.2335, 0.3096, 0.3959, 0.4771, 0.5634, 0.6446, 0.7309, 0.8223,
			0.9086, 1.0000, 1.0913, 1.1878, 1.2842, 1.3807, 1.4771, 1.5736, 1.6751, 1.7766, 1.8832,
			1.9898, 2.0964, 2.2030, 2.3147, 2.4263, 2.5380, 2.6548, 2.7715, 2.8934, 3.0101, 3.1370,
			3.2588, 3.3857, 3.5126, 3.6446,
		]
		.iter(),
	) {
		let dna_reward = compute_dna_reward::<Test>(10_000 * COIN, month) as f64 / COIN as f64;
		let dna_reward = (dna_reward * PRECISION).floor() / PRECISION;

		// eprintln!("{:?}", dna_reward);

		assert_eq!(dna_reward, *exp_dna_reward);
	}
}

#[ignore]
#[test]
fn print_total_payout_error_rate() {
	const MILLISECONDS_PER_YEAR: TsInMs = ((36525 * 24 * 60 * 60) / 100) * 1000;

	let initial_issuance = 2_000_000_000;
	let hard_cap = 10_000_000_000;
	let mut total_left = hard_cap - initial_issuance;
	let mut total_inflation = 0;

	// 100 years
	for year in 1_u32..=100 {
		let (_, inflation) = compute_total_payout::<Test>(
			MILLISECONDS_PER_YEAR,
			((year - 1) as TsInMs) * MILLISECONDS_PER_YEAR,
			total_left,
			Perbill::from_percent(0),
		);
		let inflation_rate = inflation * 10_000 / (hard_cap - total_left);

		println!(
			"year {:3}, inflation {:9}, rate {:3}",
			year, inflation, inflation_rate
		);

		total_inflation += inflation;
		total_left = total_left - inflation;
	}

	println!("total inflation: {}", total_inflation);
	println!("total left: {}", total_left);
}

#[test]
fn dna_slash_should_work() {
	let exp_slashes: [&[Balance]; 36] = [
		&[76142],
		&[152284, 76142],
		&[233502, 157360, 81218],
		&[309644, 233502, 157360, 76142],
		&[395939, 319797, 243655, 162437, 86295],
		&[477157, 401015, 324873, 243655, 167513, 81218],
		&[563451, 487309, 411167, 329949, 253807, 167512, 86294],
		&[
			644670, 568528, 492386, 411168, 335026, 248731, 167513, 81219,
		],
		&[
			730964, 654822, 578680, 497462, 421320, 335025, 253807, 167513, 86294,
		],
		&[
			822335, 746193, 670051, 588833, 512691, 426396, 345178, 258884, 177665, 91371,
		],
		&[
			908629, 832487, 756345, 675127, 598985, 512690, 431472, 345178, 263959, 177665, 86294,
		],
		&[
			1000000, 923858, 847716, 766498, 690356, 604061, 522843, 436549, 355330, 269036,
			177665, 91371,
		],
		&[
			1091370, 1015228, 939086, 857868, 781726, 695431, 614213, 527919, 446700, 360406,
			269035, 182741, 91370,
		],
		&[
			1187817, 1111675, 1035533, 954315, 878173, 791878, 710660, 624366, 543147, 456853,
			365482, 279188, 187817, 96447,
		],
		&[
			1284263, 1208121, 1131979, 1050761, 974619, 888324, 807106, 720812, 639593, 553299,
			461928, 375634, 284263, 192893, 96446,
		],
		&[
			1380710, 1304568, 1228426, 1147208, 1071066, 984771, 903553, 817259, 736040, 649746,
			558375, 472081, 380710, 289340, 192893, 96447,
		],
		&[
			1477157, 1401015, 1324873, 1243655, 1167513, 1081218, 1000000, 913706, 832487, 746193,
			654822, 568528, 477157, 385787, 289340, 192894, 96447,
		],
		&[
			1573604, 1497462, 1421320, 1340102, 1263960, 1177665, 1096447, 1010153, 928934, 842640,
			751269, 664975, 573604, 482234, 385787, 289341, 192894, 96447,
		],
		&[
			1675126, 1598984, 1522842, 1441624, 1365482, 1279187, 1197969, 1111675, 1030456,
			944162, 852791, 766497, 675126, 583756, 487309, 390863, 294416, 197969, 101522,
		],
		&[
			1776649, 1700507, 1624365, 1543147, 1467005, 1380710, 1299492, 1213198, 1131979,
			1045685, 954314, 868020, 776649, 685279, 588832, 492386, 395939, 299492, 203045,
			101523,
		],
		&[
			1883248, 1807106, 1730964, 1649746, 1573604, 1487309, 1406091, 1319797, 1238578,
			1152284, 1060913, 974619, 883248, 791878, 695431, 598985, 502538, 406091, 309644,
			208122, 106599,
		],
		&[
			1989847, 1913705, 1837563, 1756345, 1680203, 1593908, 1512690, 1426396, 1345177,
			1258883, 1167512, 1081218, 989847, 898477, 802030, 705584, 609137, 512690, 416243,
			314721, 213198, 106599,
		],
		&[
			2096446, 2020304, 1944162, 1862944, 1786802, 1700507, 1619289, 1532995, 1451776,
			1365482, 1274111, 1187817, 1096446, 1005076, 908629, 812183, 715736, 619289, 522842,
			421320, 319797, 213198, 106599,
		],
		&[
			2203045, 2126903, 2050761, 1969543, 1893401, 1807106, 1725888, 1639594, 1558375,
			1472081, 1380710, 1294416, 1203045, 1111675, 1015228, 918782, 822335, 725888, 629441,
			527919, 426396, 319797, 213198, 106599,
		],
		&[
			2314720, 2238578, 2162436, 2081218, 2005076, 1918781, 1837563, 1751269, 1670050,
			1583756, 1492385, 1406091, 1314720, 1223350, 1126903, 1030457, 934010, 837563, 741116,
			639594, 538071, 431472, 324873, 218274, 111675,
		],
		&[
			2426395, 2350253, 2274111, 2192893, 2116751, 2030456, 1949238, 1862944, 1781725,
			1695431, 1604060, 1517766, 1426395, 1335025, 1238578, 1142132, 1045685, 949238, 852791,
			751269, 649746, 543147, 436548, 329949, 223350, 111675,
		],
		&[
			2538071, 2461929, 2385787, 2304569, 2228427, 2142132, 2060914, 1974620, 1893401,
			1807107, 1715736, 1629442, 1538071, 1446701, 1350254, 1253808, 1157361, 1060914,
			964467, 862945, 761422, 654823, 548224, 441625, 335026, 223351, 111676,
		],
		&[
			2654822, 2578680, 2502538, 2421320, 2345178, 2258883, 2177665, 2091371, 2010152,
			1923858, 1832487, 1746193, 1654822, 1563452, 1467005, 1370559, 1274112, 1177665,
			1081218, 979696, 878173, 771574, 664975, 558376, 451777, 340102, 228427, 116751,
		],
		&[
			2771573, 2695431, 2619289, 2538071, 2461929, 2375634, 2294416, 2208122, 2126903,
			2040609, 1949238, 1862944, 1771573, 1680203, 1583756, 1487310, 1390863, 1294416,
			1197969, 1096447, 994924, 888325, 781726, 675127, 568528, 456853, 345178, 233502,
			116751,
		],
		&[
			2893401, 2817259, 2741117, 2659899, 2583757, 2497462, 2416244, 2329950, 2248731,
			2162437, 2071066, 1984772, 1893401, 1802031, 1705584, 1609138, 1512691, 1416244,
			1319797, 1218275, 1116752, 1010153, 903554, 796955, 690356, 578681, 467006, 355330,
			238579, 121828,
		],
		&[
			3010152, 2934010, 2857868, 2776650, 2700508, 2614213, 2532995, 2446701, 2365482,
			2279188, 2187817, 2101523, 2010152, 1918782, 1822335, 1725889, 1629442, 1532995,
			1436548, 1335026, 1233503, 1126904, 1020305, 913706, 807107, 695432, 583757, 472081,
			355330, 238579, 116751,
		],
		&[
			3137055, 3060913, 2984771, 2903553, 2827411, 2741116, 2659898, 2573604, 2492385,
			2406091, 2314720, 2228426, 2137055, 2045685, 1949238, 1852792, 1756345, 1659898,
			1563451, 1461929, 1360406, 1253807, 1147208, 1040609, 934010, 822335, 710660, 598984,
			482233, 365482, 243654, 126903,
		],
		&[
			3258883, 3182741, 3106599, 3025381, 2949239, 2862944, 2781726, 2695432, 2614213,
			2527919, 2436548, 2350254, 2258883, 2167513, 2071066, 1974620, 1878173, 1781726,
			1685279, 1583757, 1482234, 1375635, 1269036, 1162437, 1055838, 944163, 832488, 720812,
			604061, 487310, 365482, 248731, 121828,
		],
		&[
			3385786, 3309644, 3233502, 3152284, 3076142, 2989847, 2908629, 2822335, 2741116,
			2654822, 2563451, 2477157, 2385786, 2294416, 2197969, 2101523, 2005076, 1908629,
			1812182, 1710660, 1609137, 1502538, 1395939, 1289340, 1182741, 1071066, 959391, 847715,
			730964, 614213, 492385, 375634, 248731, 126903,
		],
		&[
			3512690, 3436548, 3360406, 3279188, 3203046, 3116751, 3035533, 2949239, 2868020,
			2781726, 2690355, 2604061, 2512690, 2421320, 2324873, 2228427, 2131980, 2035533,
			1939086, 1837564, 1736041, 1629442, 1522843, 1416244, 1309645, 1197970, 1086295,
			974619, 857868, 741117, 619289, 502538, 375635, 253807, 126904,
		],
		&[
			3644670, 3568528, 3492386, 3411168, 3335026, 3248731, 3167513, 3081219, 3000000,
			2913706, 2822335, 2736041, 2644670, 2553300, 2456853, 2360407, 2263960, 2167513,
			2071066, 1969544, 1868021, 1761422, 1654823, 1548224, 1441625, 1329950, 1218275,
			1106599, 989848, 873097, 751269, 634518, 507615, 385787, 258884, 131980,
		],
	];

	for promise_month in 1..=36 {
		for passed_month in 0..=promise_month - 1 {
			let dna_reward = compute_dna_reward::<Test>(10 * COIN, promise_month);
			let slashes = dna_reward - compute_dna_reward::<Test>(10 * COIN, passed_month);

			assert_eq!(
				slashes,
				exp_slashes[promise_month as usize - 1][passed_month as usize]
			);
		}
	}
}
