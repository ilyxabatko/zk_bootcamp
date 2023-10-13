### Zokrates "Hash Pre-image knowledge" proof example that can be run within the Remix IDE with the Zokrates plugin installed. 

*The example link: https://zokrates.github.io/examples/sha256example.html*

• The program takes four inputs that are used to generate a sha256 hash and then asserts the proof with the hash digest.
• The proof was generated for the following values: 0, 0, 0, 5.
• The "verifier.sol" is the autogenerated file by the ZoKrates plugin.

• The program can be compiled and deployed in the Remix IDE.
To test (call) the deployed program, one has to copy the generated proof inside the IDE and pass that data to the "verifyTx" field inside the "Deployed Contracts" tab in Remix. Potential output:
```
from	0x5B38Da6a701c568545dCfcB03FcB875f56beddC4
to	Verifier.verifyTx(((uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256)),uint256[2]) 0x7EF2e0048f5bAeDe046f6BF797943daF4ED8CB47
execution cost	227980 gas (Cost only applies when called by a contract)
input	0x43c...9ce10
decoded input	{
	"tuple proof": [
		[
			"12448302993974599570890383156770240206762360789517415611644564892907721852211",
			"15718474623048413323979795059155739432285927247510734608078826414450236401543"
		],
		[
			[
				"2502961985775562398648002375157586197226396931046008802021105894720686436680",
				"204326949027980499830214655528570085153909839950376768509945840615543300307"
			],
			[
				"13921081348906815489776560033178712918755411856057388025641712519522040320211",
				"374474486670685497119749736056696830546847393204359002291444648848716639688"
			]
		],
		[
			"2215402308459619723364005918036535171760053211167447629498783578722380129090",
			"21877007520486777278634483049968960370234880052212173357377693238545048163686"
		]
	],
	"uint256[2] input": [
		"263561599766550617289250058199814760685",
		"65303172752238645975888084098459749904"
	]
}
decoded output	{
	"0": "bool: r true"
}
logs	[]
```