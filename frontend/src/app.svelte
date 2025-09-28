<script lang="ts">
    import Sidebar from "./main/sidebar.svelte";
    import Header from "./main/header.svelte";
    import Alert from "./main/alert.svelte";

    import Dashboard from "./dashboard/site.svelte";
</script>

<div class="flex">
    <Sidebar />
    <div class="flex-1">
        <Header />
        <main class="flex-1 p-6">
            <Alert />
            <Dashboard />
        </main>
    </div>
</div>

<style>
</style>
