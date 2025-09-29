<script lang="ts">
    import { Page, currentPage } from "./page/page.svelte.ts";

    import Sidebar from "./main/sidebar.svelte";
    import Header from "./main/header.svelte";
    import Alert from "./main/alert.svelte";

    import Dashboard from "./dashboard/site.svelte";
    import Requests from "./requests/site.svelte";
    import Issues from "./issues/site.svelte";
    import Feedback from "./feedback/site.svelte";
    import Settings from "./settings/site.svelte";
</script>

<div class="flex">
    <Sidebar />
    <div class="flex-1">
        <Header />
        <main class="flex-1 p-6">
            <Alert />
            {#if currentPage.page == Page.dashboard}
                <Dashboard />
            {:else if currentPage.page == Page.requests}
                <Requests />
            {:else if currentPage.page == Page.issues}
                <Issues />
            {:else if currentPage.page == Page.feedback}
                <Feedback />
            {:else if currentPage.page == Page.settings}
                <Settings />
            {/if}
        </main>
    </div>
</div>

<style>
</style>
