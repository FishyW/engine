<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import { fetch } from '@tauri-apps/api/http';
	// $: {
	//     (async () => {
	//         if (js == null) {return;}
	//         let module = await import(/* @vite-ignore*/ js);
	//         console.log("s");
	//         module.test();
	//     })();
	// }

	let file = '';

  const textUrl = "/home/winsst/Downloads/empty.txt";
  let text = "";
	const ok = async () => {
		const res = '/home/winsst/Downloads/test.js';
		file = convertFileSrc(res, "test");
    try {
      let data = await fetch("https://test.localhost");
    //   console.log(data);

    } catch (e) {
      console.error(e);
    }
    // await fetch(convertFileSrc(res, "javascript"));
	};

  onMount(async () => {
	const path = "/home/winsst/Downloads/test.js"
	const mod = await import(`test:/${path}`);

  })
</script>


<h1>Welcome to SvelteKt!</h1>

<p>Hello</p>
<p>Test</p>

<embed src={text} />

<button on:click={ok}>Open</button>
