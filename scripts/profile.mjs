class Profile {
	constructor() {
		this.clock = process.hrtime.bigint
		this.startedAt = null
		this.label = ""
	}

	Start(label = "") {
		this.label = label
		this.startedAt = this.clock()

		if (label) {
			console.log(`--> ${label} <--`)
		}

		return this
	}

	Stop() {
		if (!this.startedAt) {
			throw new Error("Profile.Start() must be called before Stop()")
		}

		const endedAt = this.clock()
		const elapsedNs = Number(endedAt - this.startedAt)
		const elapsedMs = elapsedNs / 1e6

		console.log(`${elapsedMs.toFixed(3)} ms`)
		console.log("--------------------------------------")

		return elapsedMs
	}
}

export default Profile
