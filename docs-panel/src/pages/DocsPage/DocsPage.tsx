import { BeakerIcon, ChevronDownIcon, HandRaisedIcon, RocketLaunchIcon, TruckIcon } from '@heroicons/react/20/solid';
import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/react';
import './DocsPage.css';

export const DocsPage = () => {
  const features: { name: string; description: JSX.Element | string }[] = [
    {
      name: 'Linter',
      description: (
        <div>
          <p className="text-lg font-semibold text-white mb-4">Automatically checks your code for common issues:</p>
          <ul className="text-gray-300 list-disc ml-8 space-y-2">
            <li>Constant name must be in capitalized SNAKE_CASE</li>
            <li>Contract name must be in CamelCase</li>
            <li>Event name must be in CamelCase</li>
            <li>Function {}() must match Foundry test naming convention (for the foundry test function)</li>
            <li>Function name must be in mixedCase</li>
            <li>Function param name must be in mixedCase</li>
            <li>Modifier name must be in mixedCase</li>
            <li>{} parameter is not named (for the named parameters mapping)</li>
            <li>Only private and internal variables must start with a single underscore</li>
            <li>Avoid using letters 'I', 'l', 'O' as identifiers</li>
            <li>Variable should be in mixedCase</li>
          </ul>
        </div>
      ),
    },
    {
      name: 'Formatter',
      description:
        'Format your document with a right click and by choosing "Format Document" or by pressing Ctrl+Shift+I.',
    },
    {
      name: 'Gas estimation',
      description: (
        <div>
          <p className="inline">
            The
            <span className="bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300 inline-flex items-center mx-1">
              <RocketLaunchIcon className="mr-1 h-4 w-4" aria-hidden="true" />
              gas estimation
            </span>
            is shown on your contract after you save the file. This will show you how much it will cost to execute this
            contract.
          </p>
        </div>
      ),
    },
    {
      name: 'Slither',
      description: (
        <div>
          <p>Slither is going to report you some security issues that are in your functions and contracts:</p>
          <div className="inline-flex items-center space-x-2 mt-2">
            <HandRaisedIcon className="h-6 w-6 text-white-900" />
            <p className="text-lg font-semibold text-white">Interact</p>
          </div>
          <p>
            First of all click on our sidebar. And here, you will see two panels. You can use the first one to interact
            with a contract already deployed:
          </p>
          <ul className="text-gray-300 list-disc ml-8 space-y-2">
            <li>You have to select an account. If you do not have one you can click on the "Edit" button.</li>
            <li>
              Here, you will be able to add or remove some wallets. This is going to modify for you the wallets.json You
              will have to provide a name, an address, a privatekey and a rpc url.
            </li>
            <li>
              You will also be requested to choose a contract and a function. Finally, you put your gas limit and the
              value and you can send the transaction.
            </li>
          </ul>
          <div className="inline-flex items-center space-x-2 mt-2">
            <TruckIcon className="h-6 w-6 text-white-900" />
            <p className="text-lg font-semibold text-white">Deploy</p>
          </div>
          <p>This is the deploy part:</p>
          <ul className="text-gray-300 list-disc ml-8 space-y-2">
            <li>
              This will be the same scenario but you can deploy using a script OR using a contract. If you want to
              deploy using a script, you only have to choose the environment and the script and you can deploy it.
            </li>
            <li>
              If you want to deploy using a contrat, you will have to select an account and a contract but also an
              environment, the gas limit and the value and then you will be able to deploy it. Also, if you don’t have
              an environment, you will be able to add one with the edit button and you will just have to provide a name
              and a rpc url.
            </li>
          </ul>
        </div>
      ),
    },
    {
      name: 'Deploy & Interact sidebar',
      description:
        'First of all, click on the testing extension which is by default on VSCode. Then, in this interface, you will be able to test your files via foundry.',
    },
    // { name: 'Debugger', description: 'Debug your smart contracts.' },
    {
      name: 'Unit tests',
      description: (
        <div>
          <p>
            First of all, click on the{' '}
            <p className="text-lg inline-flex items-center mt-2">
              <span className="bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300">
                Activity Bar
              </span>
              <span className="bg-gray-100 text-gray-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-gray-700 dark:text-gray-300">
                {'>'}
              </span>
              <span className="bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300">
                <BeakerIcon className="mr-1 h-4 w-4 inline" aria-hidden="true" /> testing
              </span>
            </p>
            icon which is by default on VSCode.
          </p>
          <p>In this interface, you will be able to test your files via foundry.</p>
        </div>
      ),
    },
    {
      name: 'Compiler',
      description:
        "When you are saving your file, the compiler will try to compile your function. If there is any issue your issue will be listed in the 'Problems' section with your slither and linter issues and of course directly onyour code!",
    },
    {
      name: 'References',
      description:
        'Navigate to usages of variables. For example, if you search for "test" and it is not used anywhere else, it will say "No references found for "test". If you search for "message", it will show where it is used.',
    }
  ];

  function classNames(...classes: string[]) {
    return classes.filter(Boolean).join(' ');
  }

  return (
    <div className="flex">
      <div className="h-screen overflow-y-scroll w-full"> {/* w-3/4 */}
        <div className="max-w-5xl mx-auto px-4 sm:px-6 md:px-8 py-12 space-y-8">
          <h1 className="text-3xl font-extrabold">Osmium Extension Documentation</h1>
          {/* Settings explanation */}
          <div className="mt-8">
            <h2 className="text-xl font-semibold">Settings</h2>
            <p className="text-lg text-white-500">
              At any time during the reading of this documentation, if you feel that you won't be using certain
              features, you can deactivate them in the settings.
            </p>
            <p className="text-lg">
              To display extension settings navigate to{' '}
              <span className="bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300">
                Settings
              </span>
              <span className="bg-gray-100 text-gray-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-gray-700 dark:text-gray-300">
                {'>'}
              </span>
              <span className="bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300">
                Extension
              </span>
              <span className="bg-gray-100 text-gray-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-gray-700 dark:text-gray-300">
                {'>'}
              </span>
              <span className="bg-blue-100 text-blue-800 text-xs font-medium me-2 px-2.5 py-0.5 rounded dark:bg-blue-900 dark:text-blue-300">
                Osmium
              </span>
            </p>
          </div>

          {/* Features descriptions */}
          <div className="space-y-2 md:space-y-2">
            {features.map((feature) => (
              <Disclosure as="div" key={feature.name} defaultOpen={true}>
                {({ open }) => (
                  <>
                    <dt className="text-lg py-6">
                      <DisclosureButton
                        className={classNames(
                          open ? 'text-white-900' : 'text-white-500',
                          'group w-full flex justify-between items-start text-left',
                        )}
                      >
                        <span className="font-medium">{feature.name}</span>
                        <span className="ml-6 flex items-center">
                        <ChevronDownIcon
                          className={classNames(
                            open ? 'rotate-0' : 'rotate-90', // Rotates the icon based on open state
                            'transform transition-transform duration-300', // Smooth transition for rotation
                            'h-5 w-5 group-hover:text-white-500',
                          )}
                          aria-hidden="true"
                        />
                        </span>
                      </DisclosureButton>
                    </dt>
                    <DisclosurePanel as="dd" className="pr-12">
                      <p className="text-base text-white-500 bg-gray-800 p-6 rounded-lg shadow-lg">
                        {feature.description}
                      </p>
                    </DisclosurePanel>
                  </>
                )}
              </Disclosure>
            ))}
          </div>
        </div>
      </div>

      {/* <div className="hidden md:block bg-white-200 w-1/4">
        <div className="space-y-4 p-4">
          <h2 className="text-xl font-semibold text-white-900">Table of Contents</h2>
          <nav className="space-y-1">
            {features.map((feature) => (
              <a
                key={feature.name}
                href={`#${feature.name.toLowerCase().replace(' ', '-')}`}
                className="group flex items-center px-2 py-1 text-base font-medium rounded-md text-white-700 hover:bg-white-30 hover:text-white-900"
              >
                <span className="flex-1 ml-2">{feature.name}</span>
              </a>
            ))}
          </nav>
        </div>
      </div> */}
    </div>
  );
};