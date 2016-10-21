FactoryGirl.define do
  factory :post do
    title { Faker::Name.name }
    body { Faker::Internet.email }
    published true
  end
end
