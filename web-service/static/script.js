const btn = document.getElementById("btn");
const source = document.getElementById("source");
const result = document.getElementById("result");
btn.addEventListener("click",async ()=> {
		const a = source.value;
		const res = await fetch("/eval", {
				method: 'POST',
				cache: 'no-cache',
				body: source.value,
		}).then(res => res.text());
		result.value = res;
})

