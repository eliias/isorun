import * as React from "react";
import {ApolloClient, InMemoryCache, ApolloProvider, useQuery, gql, useApolloClient} from '@apollo/client';
import {useMemo, useRef} from "react";

const QUERY = gql`
  query TestQuery {
    testField
  }
`;

const navigation = [
  {name: 'Home', href: '#'},
  {name: 'Trending', href: '#'},
  {name: 'Bookmarks', href: '#'},
  {name: 'Messages', href: '#'},
  {name: 'Profile', href: '#'},
]
const subNavigation = [
  {
    name: 'Account',
    description: 'Ullamcorper id at suspendisse nec id volutpat vestibulum enim. Interdum blandit.',
    href: '#',
    current: true,
  },
  {
    name: 'Notifications',
    description: 'Enim, nullam mi vel et libero urna lectus enim. Et sed in maecenas tellus.',
    href: '#',
    current: false,
  },
  {
    name: 'Security',
    description: 'Semper accumsan massa vel volutpat massa. Non turpis ut nulla aliquet turpis.',
    href: '#',
    current: false,
  },
  {
    name: 'Appearance',
    description: 'Magna nulla id sed ornare ipsum eget. Massa eget porttitor suscipit consequat.',
    href: '#',
    current: false,
  },
  {
    name: 'Billing',
    description: 'Orci aliquam arcu egestas turpis cursus. Lectus faucibus netus dui auctor mauris.',
    href: '#',
    current: false,
  },
  {
    name: 'Integrations',
    description: 'Nisi, elit volutpat odio urna quis arcu faucibus dui. Mauris adipiscing pellentesque.',
    href: '#',
    current: false,
  },
  {
    name: 'Additional Resources',
    description: 'Quis viverra netus donec ut auctor fringilla facilisis. Nunc sit donec cursus sit quis et.',
    href: '#',
    current: false,
  },
]

function classNames(...classes) {
  return classes.filter(Boolean).join(' ')
}

function createClient(isSSR) {
  return new ApolloClient({
    ssrMode: false,
    uri: 'http://localhost:3000/graphql',
    cache: new InMemoryCache(),
  });
}

const UI = () => {
  const {data} = useQuery(QUERY);

  return (
    <>
      <div className="flex h-full">
        {/* Static sidebar for desktop */}
        <div className="hidden lg:flex lg:flex-shrink-0">
          <div className="flex w-20 flex-col">
            <div className="flex min-h-0 flex-1 flex-col overflow-y-auto bg-blue-600">
              <div className="flex-1">
                <div className="flex items-center justify-center bg-blue-700 py-4">
                  <img
                    className="h-8 w-auto"
                    src="https://tailwindui.com/img/logos/mark.svg?color=white"
                    alt="Your Company"
                  />
                </div>
                <nav aria-label="Sidebar" className="flex flex-col items-center space-y-3 py-6">
                  {navigation.map((item) => (
                    <a
                      key={item.name}
                      href={item.href}
                      className="flex items-center rounded-lg p-4 text-blue-200 hover:bg-blue-700"
                    >
                      <span className="sr-only">{item.name}</span>
                    </a>
                  ))}
                </nav>
              </div>
              <div className="flex flex-shrink-0 pb-5">
                <a href="#" className="w-full flex-shrink-0">
                  <img
                    className="mx-auto block h-10 w-10 rounded-full"
                    src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2.5&w=256&h=256&q=80"
                    alt=""
                  />
                  <div className="sr-only">
                    <p>Lisa Marie</p>
                    <p>Account settings</p>
                  </div>
                </a>
              </div>
            </div>
          </div>
        </div>

        <div className="flex min-w-0 flex-1 flex-col overflow-hidden">
          <main className="flex flex-1 overflow-hidden">
            <div className="flex flex-1 flex-col overflow-y-auto xl:overflow-hidden">
              {/* Breadcrumb */}
              <nav aria-label="Breadcrumb" className="border-b border-blue-gray-200 bg-white xl:hidden">
                <div className="mx-auto flex max-w-3xl items-start py-3 px-4 sm:px-6 lg:px-8">
                  <a
                    href="#"
                    className="-ml-1 inline-flex items-center space-x-3 text-sm font-medium text-blue-gray-900"
                  >
                    <span>Settings</span>
                  </a>
                </div>
              </nav>

              <div className="flex flex-1 xl:overflow-hidden">
                {/* Secondary sidebar */}
                <nav
                  aria-label="Sections"
                  className="hidden w-96 flex-shrink-0 border-r border-blue-gray-200 bg-white xl:flex xl:flex-col"
                >
                  <div className="flex h-16 flex-shrink-0 items-center border-b border-blue-gray-200 px-6">
                    <p className="text-lg font-medium text-blue-gray-900">Settings</p>
                  </div>
                  <div className="min-h-0 flex-1 overflow-y-auto">
                    {subNavigation.map((item) => (
                      <a
                        key={item.name}
                        href={item.href}
                        className={classNames(
                          item.current ? 'bg-blue-50 bg-opacity-50' : 'hover:bg-blue-50 hover:bg-opacity-50',
                          'flex p-6 border-b border-blue-gray-200'
                        )}
                        aria-current={item.current ? 'page' : undefined}
                      >
                        <div className="ml-3 text-sm">
                          <p className="font-medium text-blue-gray-900">{item.name}</p>
                          <p className="mt-1 text-blue-gray-500">{item.description}</p>
                        </div>
                      </a>
                    ))}
                  </div>
                </nav>

                {/* Main content */}
                <div className="flex-1 xl:overflow-y-auto">
                  <div className="mx-auto max-w-3xl py-10 px-4 sm:px-6 lg:py-12 lg:px-8">
                    <h1 className="text-3xl font-bold tracking-tight text-blue-gray-900">Account</h1>

                    <form className="divide-y-blue-gray-200 mt-6 space-y-8 divide-y">
                      <div className="grid grid-cols-1 gap-y-6 sm:grid-cols-6 sm:gap-x-6">
                        <div className="sm:col-span-6">
                          <h2 className="text-xl font-medium text-blue-gray-900">Profile</h2>
                          <p className="mt-1 text-sm text-blue-gray-500">
                            This information will be displayed publicly so be careful what you share.
                          </p>
                        </div>

                        <div className="sm:col-span-3">
                          <label htmlFor="first-name" className="block text-sm font-medium text-blue-gray-900">
                            First name
                          </label>
                          <input
                            type="text"
                            name="first-name"
                            id="first-name"
                            autoComplete="given-name"
                            className="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                          />
                        </div>

                        <div className="sm:col-span-3">
                          <label htmlFor="last-name" className="block text-sm font-medium text-blue-gray-900">
                            Last name
                          </label>
                          <input
                            type="text"
                            name="last-name"
                            id="last-name"
                            autoComplete="family-name"
                            className="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                          />
                        </div>

                        <div className="sm:col-span-6">
                          <label htmlFor="username" className="block text-sm font-medium text-blue-gray-900">
                            Username
                          </label>
                          <div className="mt-1 flex rounded-md shadow-sm">
                            <span
                              className="inline-flex items-center rounded-l-md border border-r-0 border-blue-gray-300 bg-blue-gray-50 px-3 text-blue-gray-500 sm:text-sm">
                              workcation.com/
                            </span>
                            <input
                              type="text"
                              name="username"
                              id="username"
                              autoComplete="username"
                              defaultValue="lisamarie"
                              className="block w-full min-w-0 flex-1 rounded-none rounded-r-md border-blue-gray-300 text-blue-gray-900 focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                            />
                          </div>
                        </div>

                        <div className="sm:col-span-6">
                          <label htmlFor="photo" className="block text-sm font-medium text-blue-gray-900">
                            Photo
                          </label>
                          <div className="mt-1 flex items-center">
                            <img
                              className="inline-block h-12 w-12 rounded-full"
                              src="https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2.5&w=256&h=256&q=80"
                              alt=""
                            />
                            <div className="ml-4 flex">
                              <div
                                className="relative flex cursor-pointer items-center rounded-md border border-blue-gray-300 bg-white py-2 px-3 shadow-sm focus-within:outline-none focus-within:ring-2 focus-within:ring-blue-500 focus-within:ring-offset-2 focus-within:ring-offset-blue-gray-50 hover:bg-blue-gray-50">
                                <label
                                  htmlFor="user-photo"
                                  className="pointer-events-none relative text-sm font-medium text-blue-gray-900"
                                >
                                  <span>Change</span>
                                  <span className="sr-only"> user photo</span>
                                </label>
                                <input
                                  id="user-photo"
                                  name="user-photo"
                                  type="file"
                                  className="absolute inset-0 h-full w-full cursor-pointer rounded-md border-gray-300 opacity-0"
                                />
                              </div>
                              <button
                                type="button"
                                className="ml-3 rounded-md border border-transparent bg-transparent py-2 px-3 text-sm font-medium text-blue-gray-900 hover:text-blue-gray-700 focus:border-blue-gray-300 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-blue-gray-50"
                              >
                                Remove
                              </button>
                            </div>
                          </div>
                        </div>

                        <div className="sm:col-span-6">
                          <label htmlFor="description" className="block text-sm font-medium text-blue-gray-900">
                            Description
                          </label>
                          <div className="mt-1">
                            <textarea
                              id="description"
                              name="description"
                              rows={4}
                              className="block w-full rounded-md border-blue-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                              defaultValue={''}
                            />
                          </div>
                          <p className="mt-3 text-sm text-blue-gray-500">
                            Brief description for your profile. URLs are hyperlinked.
                          </p>
                        </div>

                        <div className="sm:col-span-6">
                          <label htmlFor="url" className="block text-sm font-medium text-blue-gray-900">
                            URL
                          </label>
                          <input
                            type="text"
                            name="url"
                            id="url"
                            className="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                          />
                        </div>
                      </div>

                      <div className="grid grid-cols-1 gap-y-6 pt-8 sm:grid-cols-6 sm:gap-x-6">
                        <div className="sm:col-span-6">
                          <h2 className="text-xl font-medium text-blue-gray-900">Personal Information</h2>
                          <p className="mt-1 text-sm text-blue-gray-500">
                            This information will be displayed publicly so be careful what you share.
                          </p>
                        </div>

                        <div className="sm:col-span-3">
                          <label htmlFor="email-address" className="block text-sm font-medium text-blue-gray-900">
                            Email address
                          </label>
                          <input
                            type="text"
                            name="email-address"
                            id="email-address"
                            autoComplete="email"
                            className="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                          />
                        </div>

                        <div className="sm:col-span-3">
                          <label htmlFor="phone-number" className="block text-sm font-medium text-blue-gray-900">
                            Phone number
                          </label>
                          <input
                            type="text"
                            name="phone-number"
                            id="phone-number"
                            autoComplete="tel"
                            className="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                          />
                        </div>

                        <div className="sm:col-span-3">
                          <label htmlFor="country" className="block text-sm font-medium text-blue-gray-900">
                            Country
                          </label>
                          <select
                            id="country"
                            name="country"
                            autoComplete="country-name"
                            className="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                          >
                            <option/>
                            <option>United States</option>
                            <option>Canada</option>
                            <option>Mexico</option>
                          </select>
                        </div>

                        <div className="sm:col-span-3">
                          <label htmlFor="language" className="block text-sm font-medium text-blue-gray-900">
                            Language
                          </label>
                          <input
                            type="text"
                            name="language"
                            id="language"
                            className="mt-1 block w-full rounded-md border-blue-gray-300 text-blue-gray-900 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                          />
                        </div>

                        <p className="text-sm text-blue-gray-500 sm:col-span-6">
                          This account was created on{' '}
                          <time dateTime="2017-01-05T20:35:40">January 5, 2017, 8:35:40 PM</time>
                          .
                        </p>
                      </div>

                      <div className="flex justify-end pt-8">
                        <button
                          type="button"
                          className="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-blue-gray-900 shadow-sm hover:bg-blue-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        >
                          Cancel
                        </button>
                        <button
                          type="submit"
                          className="ml-3 inline-flex justify-center rounded-md border border-transparent bg-blue-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
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
    </>
  )
}

export const App = ({isSSR}) => {
  const client = useMemo(() => createClient(isSSR), []);

  return (
    <ApolloProvider client={client}>
      <UI/>
    </ApolloProvider>
  );
}
