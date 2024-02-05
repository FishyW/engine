

<script lang="ts">
    
	import { onMount } from "svelte";
    export let data;
    
    // init_script is onMount, 
    // to guarantee that init_script is called exactly once
    onMount(() => {
        console.log(data.names);
    });

    </script>
<h1>Hello</h1>
<ul>
    
{#each data.names as name}
    <li>{name}</li>
{/each}

</ul>

