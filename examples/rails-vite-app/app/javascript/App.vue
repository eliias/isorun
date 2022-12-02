<script setup lang="ts">
import {ref} from "vue";
import gql from "graphql-tag";
import {useQuery} from "@vue/apollo-composable";

const navigation = [
    {name: "Home", href: "#"},
    {name: "Trending", href: "#"},
    {name: "Bookmarks", href: "#"},
    {name: "Messages", href: "#"},
    {name: "Profile", href: "#"},
];
const subNavigation = [
    {
        name: "Account",
        description: "Ullamcorper id at suspendisse nec id volutpat vestibulum enim. Interdum blandit.",
        href: "#",
        current: true,
    },
    {
        name: "Notifications",
        description: "Enim, nullam mi vel et libero urna lectus enim. Et sed in maecenas tellus.",
        href: "#",
        current: false,
    },
    {
        name: "Security",
        description: "Semper accumsan massa vel volutpat massa. Non turpis ut nulla aliquet turpis.",
        href: "#",
        current: false,
    },
    {
        name: "Appearance",
        description: "Magna nulla id sed ornare ipsum eget. Massa eget porttitor suscipit consequat.",
        href: "#",
        current: false,
    },
    {
        name: "Billing",
        description: "Orci aliquam arcu egestas turpis cursus. Lectus faucibus netus dui auctor mauris.",
        href: "#",
        current: false,
    },
    {
        name: "Integrations",
        description: "Nisi, elit volutpat odio urna quis arcu faucibus dui. Mauris adipiscing pellentesque.",
        href: "#",
        current: false,
    },
    {
        name: "Additional Resources",
        description: "Quis viverra netus donec ut auctor fringilla facilisis. Nunc sit donec cursus sit quis et.",
        href: "#",
        current: false,
    },
];

const mobileMenuOpen = ref(false);

const query = gql`
      query {
        testField
      }
    `;
const {result, loading, error} = useQuery(query);
</script>

<script lang="ts">
export default {};
</script>

<template>
  <div class="flex h-full">
    <!-- Static sidebar for desktop -->
    <div class="hidden lg:flex lg:flex-shrink-0">
      <div class="flex w-20 flex-col">
        <div class="flex min-h-0 flex-1 flex-col overflow-y-auto bg-blue-600">
          <div class="flex-1">
            <div class="flex items-center justify-center bg-blue-700 py-4">
              <img
                class="h-8 w-auto"
                src="https://tailwindui.com/img/logos/mark.svg?color=white"
                alt="Your Company"
              >
            </div>
            <nav
              aria-label="Sidebar"
              class="flex flex-col items-center space-y-3 py-6"
            >
              <a
                v-for="item in navigation"
                :key="item.name"
                :href="item.href"
                class="flex items-center rounded-lg p-4 text-blue-200 hover:bg-blue-700"
              >
                <span class="sr-only">{{ item.name }}</span>
              </a>
            </nav>
          </div>
          <div class="flex flex-shrink-0 pb-5">
            <a
              href="#"
              class="w-full flex-shrink-0"
            >
              <img
                class="mx-auto block h-10 w-10 rounded-full"
                src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2.5&w=256&h=256&q=80"
                alt=""
              >
              <div class="sr-only">
                <p>Lisa Marie</p>
                <p>Account settings</p>
              </div>
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="flex min-w-0 flex-1 flex-col overflow-hidden">
      <!-- Mobile top navigation -->
      <div class="lg:hidden">
        <div class="flex items-center justify-between bg-blue-600 py-2 px-4 sm:px-6">
          <div>
            <img
              class="h-8 w-auto"
              src="https://tailwindui.com/img/logos/mark.svg?color=white"
              alt="Your Company"
            >
          </div>
          <div>
            <button
              type="button"
              class="-mr-3 inline-flex h-12 w-12 items-center justify-center rounded-md bg-blue-600 text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
              @click="mobileMenuOpen = true"
            >
              <span class="sr-only">Open sidebar</span>
            </button>
          </div>
        </div>
      </div>

      <main class="flex flex-1 overflow-hidden">
        <div class="flex flex-1 flex-col overflow-y-auto xl:overflow-hidden">
          <!-- Breadcrumb -->
          <nav
            aria-label="Breadcrumb"
            class="border-b border-blue-gray-200 bg-white xl:hidden"
          >
            <div class="mx-auto flex max-w-3xl items-start py-3 px-4 sm:px-6 lg:px-8">
              <a
                href="#"
                class="-ml-1 inline-flex items-center space-x-3 text-sm font-medium text-blue-gray-900"
              >
                <span>Settings</span>
              </a>
            </div>
          </nav>

          <div class="flex flex-1 xl:overflow-hidden">
            <!-- Secondary sidebar -->
            <nav
              aria-label="Sections"
              class="hidden w-96 flex-shrink-0 border-r border-blue-gray-200 bg-white xl:flex xl:flex-col"
            >
              <div class="flex h-16 flex-shrink-0 items-center border-b border-blue-gray-200 px-6">
                <p class="text-lg font-medium text-blue-gray-900">
                  Settings
                </p>
              </div>
              <div class="min-h-0 flex-1 overflow-y-auto">
                <a
                  v-for="item in subNavigation"
                  :key="item.name"
                  :href="item.href"
                  :class="[item.current ? 'bg-blue-50 bg-opacity-50' : 'hover:bg-blue-50 hover:bg-opacity-50', 'flex p-6 border-b border-blue-gray-200']"
                  :aria-current="item.current ? 'page' : undefined"
                >
                  <div class="ml-3 text-sm">
                    <p class="font-medium text-blue-gray-900">{{ item.name }}</p>
                    <p class="mt-1 text-blue-gray-500">{{ item.description }}</p>
                  </div>
                </a>
              </div>
            </nav>

            <!-- Main content -->
            <div class="flex-1 xl:overflow-y-auto">
              <div class="mx-auto max-w-3xl py-10 px-4 sm:px-6 lg:py-12 lg:px-8">
                <h1 class="text-3xl font-bold tracking-tight text-blue-gray-900">
                  Account
                </h1>
                <div>
                  <h2>Some data from GraphQL</h2>
                  <p v-if="error">
                    Something went wrong...
                  </p>
                  <p v-if="loading">
                    Loading...
                  </p>
                  <p v-else>
                    {{ result.testField }}
                  </p>
                </div>
                <form class="divide-y-blue-gray-200 mt-6 space-y-8 divide-y">
                  <div class="grid grid-cols-1 gap-y-6 sm:grid-cols-6 sm:gap-x-6">
                    <div class="sm:col-span-6">
                      <h2 class="text-xl font-medium text-blue-gray-900">
                        Profile
                      </h2>
                      <p class="mt-1 text-sm text-blue-gray-500">
                        This information will be displayed publicly so be
                        careful what you share.
                      </p>
                    </div>

                    <div class="sm:col-span-3">
                      <label
                        for="first-name"
                        class="block text-sm font-medium text-blue-gray-900"
                      >First name</label>
                      <input
                        id="first-name"
                        type="text"
                        name="first-name"
                        autocomplete="given-name"
                        class="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                      >
                    </div>

                    <div class="sm:col-span-3">
                      <label
                        for="last-name"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Last name</label>
                      <input
                        id="last-name"
                        type="text"
                        name="last-name"
                        autocomplete="family-name"
                        class="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                      >
                    </div>

                    <div class="sm:col-span-6">
                      <label
                        for="username"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Username</label>
                      <div class="mt-1 flex rounded-md shadow-sm">
                        <span
                          class="inline-flex items-center rounded-l-md border border-r-0 border-blue-gray-300 bg-blue-gray-50 px-3 text-blue-gray-500 sm:text-sm"
                        >workcation.com/</span>
                        <input
                          id="username"
                          type="text"
                          name="username"
                          autocomplete="username"
                          value="lisamarie"
                          class="block w-full min-w-0 flex-1 rounded-none rounded-r-md border-blue-gray-300 text-blue-gray-900 focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        >
                      </div>
                    </div>

                    <div class="sm:col-span-6">
                      <label
                        for="photo"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Photo</label>
                      <div class="mt-1 flex items-center">
                        <img
                          class="inline-block h-12 w-12 rounded-full"
                          src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2.5&w=256&h=256&q=80"
                          alt=""
                        >
                        <div class="ml-4 flex">
                          <div
                            class="relative flex cursor-pointer items-center rounded-md border border-blue-gray-300 bg-white py-2 px-3 shadow-sm focus-within:outline-none focus-within:ring-2 focus-within:ring-blue-500 focus-within:ring-offset-2 focus-within:ring-offset-blue-gray-50 hover:bg-blue-gray-50"
                          >
                            <label
                              for="user-photo"
                              class="pointer-events-none relative text-sm font-medium text-blue-gray-900"
                            >
                              <span>Change</span>
                              <span class="sr-only"> user photo</span>
                            </label>
                            <input
                              id="user-photo"
                              name="user-photo"
                              type="file"
                              class="absolute inset-0 h-full w-full cursor-pointer rounded-md border-gray-300 opacity-0"
                            >
                          </div>
                          <button
                            type="button"
                            class="ml-3 rounded-md border border-transparent bg-transparent py-2 px-3 text-sm font-medium text-blue-gray-900 hover:text-blue-gray-700 focus:border-blue-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-blue-gray-50"
                          >
                            Remove
                          </button>
                        </div>
                      </div>
                    </div>

                    <div class="sm:col-span-6">
                      <label
                        for="description"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Description</label>
                      <div class="mt-1">
                        <textarea
                          id="description"
                          name="description"
                          rows="4"
                          class="block w-full rounded-md border-blue-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                        />
                      </div>
                      <p class="mt-3 text-sm text-blue-gray-500">
                        Brief description for your profile. URLs are
                        hyperlinked.
                      </p>
                    </div>

                    <div class="sm:col-span-6">
                      <label
                        for="url"
                        class="block text-sm font-medium text-blue-gray-900"
                      >URL</label>
                      <input
                        id="url"
                        type="text"
                        name="url"
                        class="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                      >
                    </div>
                  </div>

                  <div class="grid grid-cols-1 gap-y-6 pt-8 sm:grid-cols-6 sm:gap-x-6">
                    <div class="sm:col-span-6">
                      <h2 class="text-xl font-medium text-blue-gray-900">
                        Personal Information
                      </h2>
                      <p class="mt-1 text-sm text-blue-gray-500">
                        This information will be displayed publicly so be
                        careful what you share.
                      </p>
                    </div>

                    <div class="sm:col-span-3">
                      <label
                        for="email-address"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Email
                        address</label>
                      <input
                        id="email-address"
                        type="text"
                        name="email-address"
                        autocomplete="email"
                        class="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                      >
                    </div>

                    <div class="sm:col-span-3">
                      <label
                        for="phone-number"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Phone
                        number</label>
                      <input
                        id="phone-number"
                        type="text"
                        name="phone-number"
                        autocomplete="tel"
                        class="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                      >
                    </div>

                    <div class="sm:col-span-3">
                      <label
                        for="country"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Country</label>
                      <select
                        id="country"
                        name="country"
                        autocomplete="country-name"
                        class="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                      >
                        <option />
                        <option>United States</option>
                        <option>Canada</option>
                        <option>Mexico</option>
                      </select>
                    </div>

                    <div class="sm:col-span-3">
                      <label
                        for="language"
                        class="block text-sm font-medium text-blue-gray-900"
                      >Language</label>
                      <input
                        id="language"
                        type="text"
                        name="language"
                        class="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                      >
                    </div>

                    <p class="text-sm text-blue-gray-500 sm:col-span-6">
                      This account was created on
                      <time datetime="2017-01-05T20:35:40">January 5, 2017, 8:35:40 PM</time>
                      .
                    </p>
                  </div>

                  <div class="flex justify-end pt-8">
                    <button
                      type="button"
                      class="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-blue-gray-900 shadow-sm hover:bg-blue-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    >
                      Cancel
                    </button>
                    <button
                      type="submit"
                      class="ml-3 inline-flex justify-center rounded-md border border-transparent bg-blue-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    >
                      Save
                    </button>
                  </div>
                </form>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>
